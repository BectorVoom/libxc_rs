//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 646/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk646<F: Float>(t301: F, t3638: F, t2031: F, t758: F, t1066: F, t287: F) -> (F, F, F, F) {
    let t3639 = t301 * t3638;
    let t3640 = t3639 * t2031;
    let t3641 = t758 * t3640;
    let t3645 = t287 * t1066;
    (t3639, t3640, t3641, t3645)
}
