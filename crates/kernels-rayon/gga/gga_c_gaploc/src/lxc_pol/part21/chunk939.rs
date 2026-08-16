//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 939/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk939(t3354: f64, t4614: f64, t597: f64, t2437: f64, t2877: f64, t2441: f64, t8072: f64, t895: f64, t4752: f64, t888: f64, t2859: f64, t10314: f64, t6717: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10327 = t4614 * t3354;
    let t10329 = 0.15337170381568299871e2_f64 * t597 * t10327;
    let t10331 = 0.35750489951850426669e0_f64 * t2437 * t2877;
    let t10334 = 0.35750489951850426669e0_f64 * t2441 * t2877;
    let t10336 = 0.35750489951850426669e0_f64 * t895 * t8072;
    let t10348 = t4752 * t888;
    let t10350 = 0.7150097990370085334e0_f64 * t2859 * t10348;
    let t10351 = t6717 * t10314;
    (t10327, t10329, t10331, t10334, t10336, t10348, t10350, t10351)
}
