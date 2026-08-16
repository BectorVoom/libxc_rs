//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 181/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk181(t213: f64, t218: f64, t607: f64, t64: f64, t215: f64, t220: f64, t43: f64, t130: f64, t139: f64, t145: f64, t459: f64, t464: f64, t458: f64, t129: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t847 = -t64 - t607;
    let t850 = piecewise3(t214, 0.0_f64, 4.0_f64 / 3.0_f64 * t215 * t847);
    let t851 = -t847;
    let t854 = piecewise3(t219, 0.0_f64, 4.0_f64 / 3.0_f64 * t220 * t851);
    let t856 = (t850 + t854) * t43;
    let t860 = t130 * t139;
    let t862 = t860 * t145 * t459;
    let t864 = t464 * t130;
    let t866 = t139 * t145 * t458;
    let t867 = t864 * t866;
    let t869 = 3.0_f64 / 128.0_f64 * t862 - t867 / 128.0_f64;
    let t871 = 1.0_f64 / t129;
    (t856, t860, t862, t864, t866, t867, t869, t871)
}
