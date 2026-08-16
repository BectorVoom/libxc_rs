//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3190/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190(t13969: f64, t19057: f64, t3506: f64, t11546: f64, t11665: f64, t11668: f64, t11692: f64, t1174: f64, t1227: f64, t15434: f64, t15622: f64, t15627: f64, t15737: f64, t18360: f64, t18584: f64, t3243: f64, t44996: f64, t45002: f64, t4582: f64, t4889: f64, t4984: f64, t52601: f64, t52813: f64, t53023: f64, t53026: f64, t53033: f64, t53238: f64, t61855: f64, t6192: f64, t6230: f64, t63415: f64) -> f64 {
    let t66241 = t3506 * t13969 * t19057;
    let t66254 = -t53023 / 1728.0_f64 + t15737 * t15622 / 768.0_f64 + t53238 * t15627 / 256.0_f64 - 5.0_f64 / 3888.0_f64 * t53026 - 7.0_f64 / 648.0_f64 * t1174 * t11546 * t63415 + 14.0_f64 / 243.0_f64 * t4889 * t15434 - t44996 * t6192 / 2304.0_f64 - t11665 * t18584 / 1152.0_f64 - t11665 * t18360 / 1152.0_f64 + t53033 / 2592.0_f64 + t66241 / 1152.0_f64 + t52813 * t4984 / 144.0_f64 + t45002 / 5184.0_f64 - 5.0_f64 / 432.0_f64 * t1227 * t4582 * t52601 * t61855 - 5.0_f64 / 13824.0_f64 * t11692 * t11668 * t6230 * t3243;
    t66254
}
