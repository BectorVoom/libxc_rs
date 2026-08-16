//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 533/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk533(t2441: f64, t2877: f64, t8072: f64, t895: f64, t4752: f64, t888: f64, t2859: f64, t10314: f64, t6717: f64, t6716: f64, t6711: f64, t6710: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10334 = 0.35750489951850426669e0_f64 * t2441 * t2877;
    let t10336 = 0.35750489951850426669e0_f64 * t895 * t8072;
    let t10348 = t4752 * t888;
    let t10350 = 0.7150097990370085334e0_f64 * t2859 * t10348;
    let t10351 = t6717 * t10314;
    let t10353 = 0.69017266717057349418e1_f64 * t6716 * t10351;
    let t10354 = t6711 * t10314;
    let t10356 = 0.11502877786176224903e2_f64 * t6710 * t10354;
    (t10334, t10336, t10348, t10350, t10353, t10356)
}
