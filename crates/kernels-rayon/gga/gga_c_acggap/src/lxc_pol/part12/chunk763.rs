//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 763/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk763(t7462: f64, t7515: f64, t7519: f64, t7539: f64, t7464: f64, t7466: f64, t7468: f64, t7473: f64, t7479: f64, t7481: f64, t7484: f64, t7488: f64, t7491: f64, t7496: f64, t7500: f64, t7504: f64, t7524: f64, t7529: f64, t7531: f64, t7536: f64) -> (f64, f64, f64, f64, f64) {
    let t8171 = 0.28582678745379824648e-3_f64 * t7462;
    let t8184 = 0.85748036236139473944e-3_f64 * t7515;
    let t8185 = 0.12579236915841660827e-2_f64 * t7519;
    let t8190 = 0.62896184579208304138e-3_f64 * t7539;
    let t8191 = -t8171 + 0.75475421495049964965e-2_f64 * t7464 - 0.11321313224257494745e-1_f64 * t7466 + 0.31448092289604152068e-2_f64 * t7468 + 0.15724046144802076034e-2_f64 * t7473 + 0.20965394859736101379e-2_f64 * t7479 - 0.12579236915841660828e-2_f64 * t7481 + 0.916875e-1_f64 * t7484 + 0.61125e-1_f64 * t7488 + 0.305625e-1_f64 * t7491 - 0.31448092289604152068e-2_f64 * t7496 + 0.12579236915841660828e-2_f64 * t7500 - 0.916875e-1_f64 * t7504 + t8184 - t8185 - 0.62896184579208304138e-3_f64 * t7524 - 0.83861579438944405517e-3_f64 * t7529 + 0.18868855373762491241e-2_f64 * t7531 + 0.94344276868812456207e-3_f64 * t7536 + t8190;
    (t8171, t8184, t8185, t8190, t8191)
}
