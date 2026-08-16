//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2597/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2597(t18281: f64, t189: f64, t18555: f64, t2619: f64, t14341: f64, t4311: f64, t18562: f64, t2516: f64, t2496: f64, t5825: f64, t749: f64, t4401: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61266 = t189 * t18281;
    let t61282 = t18555 * t2619;
    let t61289 = t4311 * t14341;
    let t61294 = t18562 * t2516;
    let t61296 = t18562 * t2496;
    let t61303 = t749 * t5825;
    let t61305 = t4401 * t61303 * t606;
    (t61266, t61282, t61289, t61294, t61296, t61303, t61305)
}
