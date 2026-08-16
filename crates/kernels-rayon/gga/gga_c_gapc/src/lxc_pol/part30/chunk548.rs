//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 548/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk548(t169: f64, t588: f64, t125: f64, t481: f64, t173: f64, t1013: f64, t605: f64, t1051: f64, t731: f64, t763: f64, t282: f64, t932: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3170 = t169 * t588;
    let t3171 = t481 * t125;
    let t3172 = t3171 * t173;
    let t3173 = t3170 * t3172;
    let t3175 = t1013 * t605;
    let t3182 = t731 * t1051;
    let t3184 = t763 * t1051;
    let t3186 = t932 * t282;
    (t3170, t3171, t3172, t3173, t3175, t3182, t3184, t3186)
}
