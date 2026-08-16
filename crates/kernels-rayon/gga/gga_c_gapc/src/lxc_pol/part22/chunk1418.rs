//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1418/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1418(t35280: f64, t35283: f64, t35285: f64, t35293: f64, t35298: f64, t35287: f64, t35289: f64, t37260: f64, t37261: f64, t37262: f64, t37263: f64, t35302: f64) -> (f64, f64) {
    let t37264 = 0.33816362383187442026e-5_f64 * t35280;
    let t37265 = 0.16038463156432184077e-5_f64 * t35283;
    let t37266 = 0.12661944597183303218e-6_f64 * t35285;
    let t37269 = 0.18937162934584967535e-3_f64 * t35293;
    let t37270 = 0.18937162934584967535e-3_f64 * t35298;
    let t37271 = t37260 + t37261 + t37262 - t37263 - t37264 + t37265 + t37266 - 0.38673709012042260327e-7_f64 * t35287 - 0.54083013361612955739e-6_f64 * t35289 - t37269 + t37270;
    let t37273 = 0.21642471925239962898e-3_f64 * t35302;
    (t37271, t37273)
}
