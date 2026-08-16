//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 868/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk868(t1086: f64, t2628: f64, t10046: f64, t2233: f64, t2982: f64, t3387: f64, t3138: f64, t3363: f64, t3330: f64, t818: f64, t959: f64, t3329: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t10047 = t1086 * t2628;
    let t10048 = t10046 * t10047;
    let t10050 = t2982 * t2233;
    let t10051 = t3387 * t10050;
    let t10053 = t3363 * t3138;
    let t10054 = t10053 * t3330;
    let t10057 = pi * t818 * t959;
    let t10058 = t10057 * t3329;
    (t10047, t10048, t10051, t10054, t10057, t10058)
}
