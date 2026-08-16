//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 756/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk756(t22819: f64, t22842: f64, t3076: f64, t32187: f64, t32279: f64, t32281: f64, t32284: f64, t32289: f64, t32292: f64, t32295: f64, t32297: f64, t32301: f64, t32304: f64, t32308: f64, t32313: f64, t32316: f64, t32318: f64, t385: f64, t399: f64, t7173: f64, t7181: f64, t7183: f64, t7202: f64) -> f64 {
    let t32322 = 0.39129660776942540761e-2_f64 * t32279 * t32281 - 0.68116566383613497688e-3_f64 * t22819 * t32284 - 0.11854761295685025975e-1_f64 * t7181 * t32187 + 0.22227677429409423704e-2_f64 * t32289 * t7183 + 0.88910709717637694816e-2_f64 * t22842 * t32292 - 0.25845121844514357744e-4_f64 * t32295 * t32297 - 0.23254900946437792e-1_f64 * t32301 * t385 + 0.25845121844514357744e-4_f64 * t32304 * t32297 - 0.31303728621554032609e-1_f64 * t7202 * t32308 + t32313 + 0.11854761295685025975e-1_f64 * t7173 * t399 + 0.1443087735596363459e-7_f64 * t3076 * t32316 * t32318;
    t32322
}
