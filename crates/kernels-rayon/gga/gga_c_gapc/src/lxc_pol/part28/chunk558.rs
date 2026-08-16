//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 558/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk558(t169: f64, t3153: f64, t3157: f64, t1012: f64, t561: f64, t182: f64, t1667: f64, t3017: f64, t1043: f64, t1019: f64, t3080: f64, t1040: f64, t3121: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3158 = t169 * t3153 * t3157;
    let t3160 = t561 * t1012;
    let t3161 = t3160 * t182;
    let t3163 = t3017 * t1667;
    let t3164 = t1043 * t3163;
    let t3166 = t3080 * t1019;
    let t3168 = t3121 * t1040;
    (t3158, t3160, t3161, t3163, t3164, t3166, t3168)
}
