//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2085/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2085<F: Float>(t23473: F, t82943: F, t1933: F, t23479: F, t82921: F, t23433: F, t3103: F, t10889: F, t23535: F, t3033: F, t10908: F, t6755: F) -> (F, F, F, F, F) {
    let t82944 = t82943 * t23473;
    let t82951 = t1933 * t82921 * t23479;
    let t82953 = t23433 * t3103;
    let t82956 = t3033 * t23535 * t10889;
    let t82961 = t6755 * t10908;
    (t82944, t82951, t82953, t82956, t82961)
}
