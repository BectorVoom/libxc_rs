//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 275/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk275(t241: f64, t252: f64, t776: f64, t802: f64, t805: f64, t810: f64, t819: f64, t825: f64, t829: f64, t838: f64, t256: f64) -> (f64, f64, f64) {
    let t842 = t241 * (-0.3109e-1_f64 * t805 * t252 + 1.0_f64 * t810 * t819 + t776 - t802 - 0.19751789702565206229e-1_f64 * t825 + 0.58482233974552040708e0_f64 * t829 * t838);
    let t844 = 0.19751789702565206229e-1_f64 * t241 * t825;
    let t845 = t241 * t256;
    (t842, t844, t845)
}
