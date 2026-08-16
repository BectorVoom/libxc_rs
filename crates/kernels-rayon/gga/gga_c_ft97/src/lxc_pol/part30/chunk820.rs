//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 820/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk820(t299: f64, t34259: f64, t34336: f64, t332: f64, t5: f64, t7691: f64, t113: f64, t505: f64, t7692: f64, t911: f64, t1091: f64, t33535: f64, t2354: f64) -> (f64, f64, f64, f64, f64) {
    let t300 = 10000000.0_f64 <= t299;
    let t34337 = t34259 + t34336;
    let t34338 = t34337 * t332;
    let t34341 = t5 * t7691;
    let t34347 = piecewise3(t300, 0.0_f64, t5 * t34338 * t113 / 4.0_f64 + t5 * t7692 * t505 / 4.0_f64 + t34341 * t911 / 4.0_f64);
    let t35250 = t33535 * t1091;
    let t35251 = t2354 * t35250;
    (t34337, t34338, t34341, t34347, t35251)
}
