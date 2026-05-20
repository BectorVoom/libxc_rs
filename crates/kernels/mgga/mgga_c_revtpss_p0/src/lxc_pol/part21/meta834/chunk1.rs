//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3123/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3123<F: Float>(t1012: F, t44958: F, t13026: F, t140: F, t1222: F, t16715: F, t1224: F, t5052: F, t697: F, t1042: F, t1261: F, t12784: F, t12789: F, t12910: F, t12991: F, t17448: F, t17459: F, t17600: F, t17690: F, t17763: F, t3640: F, t3644: F, t3720: F, t5268: F, t5340: F, t5341: F, t53474: F, t54450: F, t56172: F, t56254: F, t57373: F, t57435: F, t57449: F, t57451: F, t57464: F, t57466: F, t57471: F, t57473: F, t57478: F) -> F {
    let t57480 = t1012 * t44958;
    let t57484 = t140 * t13026;
    let t57486 = t1222 * t57484 * t16715;
    let t57490 = t1222 * t697 * t1224 * t5052;
    let t57491 = t57490 / F::new(216.0);
    let t57496 = F::cast_from(0.42874018118069736972e-3_f64) * t57435 - F::cast_from(0.28582678745379824648e-3_f64) * t1261 * t1042 * t5268 * t54450 - F::cast_from(0.34299214494455789578e-2_f64) * t1261 * t1042 * t56254 * t53474 - F::cast_from(0.42874018118069736972e-3_f64) * t17763 * t3640 - F::cast_from(0.85748036236139473944e-3_f64) * t17763 * t3644 - F::cast_from(0.28582678745379824648e-3_f64) * t57449 - F::cast_from(0.57165357490759649295e-3_f64) * t57451 + F::cast_from(0.12862205435420921092e-2_f64) * t5340 * t3720 * t57373 * t5341 + F::cast_from(0.12862205435420921092e-2_f64) * t12910 * t3720 * t17600 * t17459 + t57464 - F::cast_from(0.12862205435420921092e-2_f64) * t57466 * t12991 - F::cast_from(0.63517063878621832551e-4_f64) * t57471 + F::cast_from(0.68598428988911579154e-2_f64) * t57473 * t12991 - F::cast_from(0.28582678745379824648e-3_f64) * t57478 + F::new(35.0) / F::new(972.0) * t1222 * t57480 * t56172 - F::new(7.0) / F::new(648.0) * t57486 + t57491 + F::cast_from(0.7145669686344956162e-3_f64) * t12784 * t17690 + F::cast_from(0.7145669686344956162e-3_f64) * t17448 * t12789;
    t57496
}
