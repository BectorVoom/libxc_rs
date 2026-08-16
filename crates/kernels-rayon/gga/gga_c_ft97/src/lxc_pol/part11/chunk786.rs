//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 786/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk786(t10279: f64, t10282: f64, t10259: f64, t10265: f64, t10269: f64, t10273: f64, t10391: f64, t10552: f64, t10553: f64, t10555: f64, t10624: f64, t10634: f64, t10636: f64) -> f64 {
    let t10640 = 4.0_f64 / 27.0_f64 * t10279;
    let t10641 = t10282 / 9.0_f64;
    let t10642 = -t10391 / 3.0_f64 + t10552 - t10553 - 2.0_f64 * t10265 - t10555 + t10624 / 6.0_f64 + t10634 / 8.0_f64 - t10636 - t10259 / 9.0_f64 + 2.0_f64 * t10269 - 10.0_f64 / 81.0_f64 * t10273 - t10640 + t10641;
    t10642
}
