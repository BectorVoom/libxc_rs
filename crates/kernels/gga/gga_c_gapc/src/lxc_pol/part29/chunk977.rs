//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 977/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk977<F: Float>(t291: F, t3137: F, t959: F, t7191: F, t11834: F, t1026: F, t932: F, t3304: F, t3285: F, t3775: F, t3289: F, t19: F, t825: F) -> (F, F, F, F, F, F, F) {
    let t11836 = t3137 * t291 * t959;
    let t11837 = t11836 * t7191;
    let t11838 = t11834 * t11837;
    let t11840 = t932 * t1026;
    let t11841 = t11840 * t3304;
    let t11843 = t3775 * t3285;
    let t11845 = t3775 * t3289;
    let t11847 = t825 * t19;
    (t11837, t11838, t11840, t11841, t11843, t11845, t11847)
}
