//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3233/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3233(t50852: f64, t50856: f64, t18562: f64, t2516: f64, t2496: f64, t18305: f64, t2258: f64, t4401: f64, t14325: f64, t18306: f64, t5825: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t61292 = 0.10389515463408878255e3_f64 * t50852;
    let t61293 = 0.11393789434848516923e-2_f64 * t50856;
    let t61294 = t18562 * t2516;
    let t61295 = 0.5848223622634646207e0_f64 * t61294;
    let t61296 = t18562 * t2496;
    let t61297 = 0.17315859105681463759e2_f64 * t61296;
    let t61300 = 12.0_f64 * t4401 * t18305 * t2258;
    let t61302 = 24.0_f64 * t14325 * t18306;
    let t61303 = t749 * t5825;
    (t61292, t61293, t61295, t61297, t61300, t61302, t61303)
}
