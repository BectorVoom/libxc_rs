//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 150/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk150(t293: f64, t291: f64, t135: f64, t286: f64, t455: f64, t458: f64, t456: f64, t708: f64, t295: f64, t471: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t711 = t293 * t293;
    let t712 = 1.0_f64 / t711;
    let t713 = t291 * t712;
    let t714 = t713 * t135;
    let t716 = t455 * t286 * t458;
    let t719 = -7.0_f64 / 128.0_f64 * t456 * t286 * t708 + 7.0_f64 / 384.0_f64 * t714 * t716;
    let t723 = t719 * t471 - 4.0_f64 / 3.0_f64 * t295 * t64;
    (t711, t712, t713, t716, t723)
}
