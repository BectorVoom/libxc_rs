//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3198/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198(t3545: f64, t6109: f64, t13969: f64, t19071: f64, t3515: f64, t11728: f64, t18306: f64, t11738: f64, t19076: f64, t11692: f64, t1174: f64, t1177: f64, t1196: f64, t1227: f64, t15239: f64, t15453: f64, t15507: f64, t15531: f64, t15667: f64, t1735: f64, t3248: f64, t3252: f64, t3506: f64, t3508: f64, t3578: f64, t45224: f64, t4582: f64, t4889: f64, t4954: f64, t4977: f64, t52615: f64, t53360: f64, t55677: f64, t61910: f64, t6230: f64, t63402: f64, t66310: f64, t974: f64) -> f64 {
    let t66500 = t6109 * t3545;
    let t66512 = t3515 * t13969 * t19071;
    let t66515 = t11728 * t13969 * t18306;
    let t66518 = t11738 * t13969 * t19076;
    let t66528 = t11692 * t3578 * t6230 * t3252 / 4608.0_f64 + t11692 * t3578 * t6230 * t3248 / 2304.0_f64 + t52615 * t4954 / 216.0_f64 + t3506 * t4582 * t4977 * t3508 * t15239 / 768.0_f64 - 5.0_f64 / 5184.0_f64 * t1227 * t4582 * t15453 * t61910 - 11.0_f64 / 486.0_f64 * t66500 + t11692 * t3578 * t1735 * t66310 / 1152.0_f64 + t15507 * t15531 / 288.0_f64 - t1174 * t1177 * t63402 / 48.0_f64 - t66512 / 1152.0_f64 - t66515 / 384.0_f64 + t66518 / 2304.0_f64 - t45224 / 13824.0_f64 + t4889 * t15667 / 54.0_f64 - t1174 * t974 * t1196 * t55677 / 288.0_f64 - 5.0_f64 / 1944.0_f64 * t53360;
    t66528
}
