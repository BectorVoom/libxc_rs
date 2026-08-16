//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1922/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1922<F: Float>(t16828: F, t1888: F, t6646: F, t1484: F, t1519: F, t25038: F, t25248: F, t776: F, t232: F, t58262: F, t23110: F, t23185: F, t28422: F) -> (F, F, F, F, F) {
    let t98387 = t1888 * t6646 * t16828;
    let t98389 = t1519 * t1484;
    let t98392 = t25038 * t25248 * t98389 * t776;
    let t98396 = t1888 * t6646 * t58262 * t232;
    let t98399 = t23185 * t23110 * t28422;
    (t98387, t98389, t98392, t98396, t98399)
}
