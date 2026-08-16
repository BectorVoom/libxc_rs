//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1190/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1190(t12915: f64, t247: f64, t34964: f64, t8926: f64, t1243: f64, t1828: f64, t105460: f64, t1203: f64, t1214: f64, t124605: f64, t124659: f64, t124694: f64, t124706: f64, t124772: f64, t1248: f64, t124903: f64, t125017: f64, t1287: f64, t29187: f64, t33449: f64, t33462: f64, t33469: f64, t33484: f64, t33485: f64, t34914: f64, t34949: f64, t34960: f64, t473: f64, t5284: f64, t5497: f64, t7627: f64, t7637: f64, t8201: f64, t8932: f64) -> f64 {
    let t131907 = t8926 * t247 * t12915 * t34964;
    let t131920 = t1243 * t1828;
    let t131925 = -0.3427184259906141157e1_f64 * t33469 * t33462 * t8201 * t7627 - 0.34694512752820797848e1_f64 * t124694 * t29187 + 0.11423947533020470523e1_f64 * t124903 * t34949 + 0.11423947533020470523e1_f64 * t33484 * t33485 * t5284 * t1287 + 0.3427184259906141157e1_f64 * t124659 * t33462 * t34914 * t1203 - 0.51407763898592117355e1_f64 * t124706 * t33462 * t34914 * t1214 - 0.18822977838986977999e-3_f64 * t131907 + 0.34694512752820797848e1_f64 * t124605 * t7637 * t105460 - 0.3427184259906141157e1_f64 * t125017 * t34960 * t1248 * t1287 + 0.17347256376410398924e1_f64 * t8932 * t33449 * t473 * t5497 + 0.17347256376410398924e1_f64 * t124772 * t131920 * t1248 * t1287;
    t131925
}
