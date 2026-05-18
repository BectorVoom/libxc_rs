//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 829/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk829<F: Float>(t1084: F, t8838: F, t147: F, t291: F, t329: F, t3413: F, t7122: F, t9245: F, t3404: F, t959: F, t3328: F, t7115: F) -> (F, F, F, F, F, F) {
    let t9919 = t1084 * t8838;
    let t9920 = t147 * t291;
    let t9921 = t9920 * t329;
    let t9922 = t7122 * t3413;
    let t9923 = t9921 * t9922;
    let t9924 = t9919 * t9923;
    let t9926 = t1084 * t9245;
    let t9927 = t3404 * t959;
    let t9928 = t7115 * t3328;
    (t9921, t9923, t9924, t9926, t9927, t9928)
}
