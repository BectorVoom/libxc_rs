//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 756/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk756<F: Float>(t147: F, t291: F, t329: F, t3413: F, t7122: F, t9919: F, t1084: F, t9245: F, t3404: F, t959: F, t3328: F, t7115: F, t3402: F, t9253: F, t1038: F, t8140: F) -> (F, F, F, F, F, F, F) {
    let t9920 = t147 * t291;
    let t9921 = t9920 * t329;
    let t9922 = t7122 * t3413;
    let t9923 = t9921 * t9922;
    let t9924 = t9919 * t9923;
    let t9926 = t1084 * t9245;
    let t9927 = t3404 * t959;
    let t9928 = t7115 * t3328;
    let t9929 = t9927 * t9928;
    let t9930 = t9926 * t9929;
    let t9932 = t3402 * t9253;
    let t9933 = t1038 * t8140;
    (t9921, t9923, t9924, t9929, t9930, t9932, t9933)
}
