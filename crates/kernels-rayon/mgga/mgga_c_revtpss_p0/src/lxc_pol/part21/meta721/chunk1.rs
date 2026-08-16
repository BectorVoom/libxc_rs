//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2561/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2561(t527: f64, t9615: f64, t1340: f64, t40165: f64, t2626: f64, t9551: f64, t512: f64, t749: f64, t9363: f64, t268: f64, t520: f64, t39768: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47040 = 1.0_f64 / t527 / t9615;
    let t47059 = 0.12304822629859687989e5_f64 * t1340 * t40165;
    let t47060 = t9551 * t2626;
    let t47063 = t512 * t9363 * t749;
    let t47065 = t520 * t268;
    let t47067 = 0.19263893255070628431e1_f64 * t47065 * t39768;
    (t47040, t47059, t47060, t47063, t47065, t47067)
}
