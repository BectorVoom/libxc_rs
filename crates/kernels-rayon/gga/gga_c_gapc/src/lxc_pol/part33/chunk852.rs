//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 852/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk852(t3322: f64, t9916: f64, t1084: f64, t8838: f64, t147: f64, t291: f64, t329: f64, t3413: f64, t7122: f64, t9245: f64, t3404: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9917 = t9916 * t3322;
    let t9919 = t1084 * t8838;
    let t9920 = t147 * t291;
    let t9921 = t9920 * t329;
    let t9922 = t7122 * t3413;
    let t9923 = t9921 * t9922;
    let t9924 = t9919 * t9923;
    let t9926 = t1084 * t9245;
    let t9927 = t3404 * t959;
    (t9917, t9921, t9923, t9924, t9926, t9927)
}
