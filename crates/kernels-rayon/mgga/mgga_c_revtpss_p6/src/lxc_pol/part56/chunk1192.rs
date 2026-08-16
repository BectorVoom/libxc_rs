//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1192/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1192(t1243: f64, t124694: f64, t124772: f64, t124780: f64, t124931: f64, t124984: f64, t124996: f64, t125017: f64, t1276: f64, t1287: f64, t1294: f64, t131699: f64, t131703: f64, t131962: f64, t247: f64, t29166: f64, t29233: f64, t29247: f64, t29301: f64, t33456: f64, t33462: f64, t33469: f64, t33478: f64, t33480: f64, t34939: f64, t3719: f64, t5245: f64, t5284: f64, t5458: f64, t5497: f64, t8926: f64, t8931: f64) -> f64 {
    let t132005 = 0.17347256376410398924e1_f64 * t124996 * t131962 * t5458 - 0.17347256376410398924e1_f64 * t124780 * t29247 + 0.17347256376410398924e1_f64 * t124772 * t131962 * t29166 + 0.51407763898592117355e1_f64 * t33469 * t33478 * t34939 * t1294 + 0.34694512752820797848e1_f64 * t124931 * t29233 - 0.17135921299530705785e1_f64 * t33469 * t33462 * t8931 * t5245 - 0.34271842599061411569e1_f64 * t124984 * t131699 * t5458 - 0.34271842599061411569e1_f64 * t125017 * t131699 * t29166 - 0.34694512752820797848e1_f64 * t124694 * t29301 - 0.8673628188205199462e0_f64 * t33456 * t1243 * t5284 * t1287 - 0.17135921299530705785e1_f64 * t131703 * t33480 - 0.28234466758480466999e-3_f64 * t8926 * t247 * t3719 * t1276 * t5497;
    t132005
}
