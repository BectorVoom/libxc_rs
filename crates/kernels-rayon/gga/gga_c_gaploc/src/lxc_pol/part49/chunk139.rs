//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 139/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk139(t211: f64, t90: f64, t238: f64, t233: f64, t345: f64, t347: f64, t351: f64, t353: f64, t241: f64, t367: f64, t46: f64, t372: f64, t374: f64) -> (f64, f64, f64, f64, f64) {
    let t607 = t211 * t90;
    let t622 = t238 * t238;
    let t623 = 1.0_f64 / t622;
    let t624 = t233 * t623;
    let t629 = -0.1176575e1_f64 * t345 - 0.516475e0_f64 * t347 - 0.2103875e0_f64 * t351 - 0.104195e0_f64 * t353;
    let t630 = 1.0_f64 / t241;
    let t631 = t629 * t630;
    let t637 = t46 * t367;
    let t638 = t372 * t374;
    (t607, t624, t631, t637, t638)
}
