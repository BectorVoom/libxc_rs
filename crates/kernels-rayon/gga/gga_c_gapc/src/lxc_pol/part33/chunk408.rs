//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 408/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk408(t238: f64, t704: f64, t233: f64, t711: f64, t712: f64, t1165: f64, t1167: f64, t1169: f64, t1197: f64, t1199: f64, t1201: f64, t241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2088 = t704 * t238;
    let t2089 = 1.0_f64 / t2088;
    let t2090 = t233 * t2089;
    let t2091 = t711 * t711;
    let t2092 = t2091 * t712;
    let t2101 = -0.78438333333333333333e0_f64 * t1165 + 0.15687666666666666667e1_f64 * t1167 + 0.68863333333333333333e0_f64 * t1169 + 0.14025833333333333333e0_f64 * t1197 + 0.28051666666666666667e0_f64 * t1199 + 0.17365833333333333333e0_f64 * t1201;
    let t2102 = t2101 * t712;
    let t2105 = t704 * t704;
    let t2106 = 1.0_f64 / t2105;
    let t2107 = t233 * t2106;
    let t2108 = t241 * t241;
    (t2090, t2091, t2092, t2102, t2107, t2108)
}
