//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 139/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk139(t213: f64, t218: f64, t211: f64, t90: f64, t64: f64, t215: f64, t220: f64, t43: f64, t238: f64, t233: f64, t345: f64, t347: f64, t351: f64, t353: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t607 = t211 * t90;
    let t608 = t64 - t607;
    let t611 = piecewise3(t214, 0.0_f64, 4.0_f64 / 3.0_f64 * t215 * t608);
    let t612 = -t608;
    let t615 = piecewise3(t219, 0.0_f64, 4.0_f64 / 3.0_f64 * t220 * t612);
    let t617 = (t611 + t615) * t43;
    let t622 = t238 * t238;
    let t623 = 1.0_f64 / t622;
    let t624 = t233 * t623;
    let t629 = -0.1176575e1_f64 * t345 - 0.516475e0_f64 * t347 - 0.2103875e0_f64 * t351 - 0.104195e0_f64 * t353;
    (t617, t624, t629)
}
