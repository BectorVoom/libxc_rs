//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 201/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk201(t213: f64, t218: f64, t211: f64, t88: f64, t62: f64, t215: f64, t220: f64, t43: f64, t238: f64, t233: f64, t352: f64, t354: f64, t358: f64, t360: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t689 = t211 * t88;
    let t690 = t62 - t689;
    let t693 = piecewise3(t214, 0.0_f64, 4.0_f64 / 3.0_f64 * t215 * t690);
    let t694 = -t690;
    let t697 = piecewise3(t219, 0.0_f64, 4.0_f64 / 3.0_f64 * t220 * t694);
    let t699 = (t693 + t697) * t43;
    let t704 = t238 * t238;
    let t705 = 1.0_f64 / t704;
    let t706 = t233 * t705;
    let t711 = -0.1176575e1_f64 * t352 - 0.516475e0_f64 * t354 - 0.2103875e0_f64 * t358 - 0.104195e0_f64 * t360;
    (t689, t690, t694, t699, t704, t705, t706, t711)
}
