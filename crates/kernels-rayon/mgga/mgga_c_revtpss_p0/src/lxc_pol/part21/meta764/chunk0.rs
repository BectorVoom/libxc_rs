//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2712/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2712(t4186: f64, t4401: f64, t606: f64, t749: f64, t39737: f64, t39766: f64, t10433: f64, t4311: f64, t10489: f64, t2403: f64, t39741: f64, t39744: f64, t39747: f64, t39750: f64, t39756: f64, t39760: f64, t39764: f64, t39770: f64, t4546: f64) -> (f64, f64, f64, f64, f64) {
    let t49911 = t4401 * t749 * t4186 * t606;
    let t49912 = 72.0_f64 * t49911;
    let t49913 = 24.0_f64 * t39737;
    let t49918 = 12.0_f64 * t39766;
    let t49920 = 4.0_f64 * t4311 * t10433;
    let t49921 = 3.0_f64 * t10489 * t2403 * t4546 + t39741 + t39744 + t39747 + t39750 + t39756 + t39760 - t39764 + t39770 + t49918 + t49920;
    (t49912, t49913, t49918, t49920, t49921)
}
