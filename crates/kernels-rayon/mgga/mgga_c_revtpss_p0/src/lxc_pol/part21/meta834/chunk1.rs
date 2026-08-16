//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3123/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123(t1012: f64, t44958: f64, t13026: f64, t140: f64, t1222: f64, t16715: f64, t1224: f64, t5052: f64, t697: f64, t1042: f64, t1261: f64, t12784: f64, t12789: f64, t12910: f64, t12991: f64, t17448: f64, t17459: f64, t17600: f64, t17690: f64, t17763: f64, t3640: f64, t3644: f64, t3720: f64, t5268: f64, t5340: f64, t5341: f64, t53474: f64, t54450: f64, t56172: f64, t56254: f64, t57373: f64, t57435: f64, t57449: f64, t57451: f64, t57464: f64, t57466: f64, t57471: f64, t57473: f64, t57478: f64) -> f64 {
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    let t57486 = t1222 * t57484 * t16715;
    let t57490 = t1222 * t697 * t1224 * t5052;
    let t57491 = t57490 / 216.0_f64;
    let t57496 = 0.42874018118069736972e-3_f64 * t57435 - 0.28582678745379824648e-3_f64 * t1261 * t1042 * t5268 * t54450 - 0.34299214494455789578e-2_f64 * t1261 * t1042 * t56254 * t53474 - 0.42874018118069736972e-3_f64 * t17763 * t3640 - 0.85748036236139473944e-3_f64 * t17763 * t3644 - 0.28582678745379824648e-3_f64 * t57449 - 0.57165357490759649295e-3_f64 * t57451 + 0.12862205435420921092e-2_f64 * t5340 * t3720 * t57373 * t5341 + 0.12862205435420921092e-2_f64 * t12910 * t3720 * t17600 * t17459 + t57464 - 0.12862205435420921092e-2_f64 * t57466 * t12991 - 0.63517063878621832551e-4_f64 * t57471 + 0.68598428988911579154e-2_f64 * t57473 * t12991 - 0.28582678745379824648e-3_f64 * t57478 + 35.0_f64 / 972.0_f64 * t1222 * t57480 * t56172 - 7.0_f64 / 648.0_f64 * t57486 + t57491 + 0.7145669686344956162e-3_f64 * t12784 * t17690 + 0.7145669686344956162e-3_f64 * t17448 * t12789;
    t57496
}
