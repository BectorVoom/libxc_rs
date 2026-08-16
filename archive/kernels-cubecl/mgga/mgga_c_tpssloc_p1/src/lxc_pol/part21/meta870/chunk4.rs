//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3198/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3198<F: Float>(t3545: F, t6109: F, t13969: F, t19071: F, t3515: F, t11728: F, t18306: F, t11738: F, t19076: F, t11692: F, t1174: F, t1177: F, t1196: F, t1227: F, t15239: F, t15453: F, t15507: F, t15531: F, t15667: F, t1735: F, t3248: F, t3252: F, t3506: F, t3508: F, t3578: F, t45224: F, t4582: F, t4889: F, t4954: F, t4977: F, t52615: F, t53360: F, t55677: F, t61910: F, t6230: F, t63402: F, t66310: F, t974: F) -> F {
    let t66500 = t6109 * t3545;
    let t66512 = t3515 * t13969 * t19071;
    let t66515 = t11728 * t13969 * t18306;
    let t66518 = t11738 * t13969 * t19076;
    let t66528 = t11692 * t3578 * t6230 * t3252 / F::cast_from(4608.0_f64) + t11692 * t3578 * t6230 * t3248 / F::cast_from(2304.0_f64) + t52615 * t4954 / F::cast_from(216.0_f64) + t3506 * t4582 * t4977 * t3508 * t15239 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1227 * t4582 * t15453 * t61910 - F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t66500 + t11692 * t3578 * t1735 * t66310 / F::cast_from(1152.0_f64) + t15507 * t15531 / F::cast_from(288.0_f64) - t1174 * t1177 * t63402 / F::cast_from(48.0_f64) - t66512 / F::cast_from(1152.0_f64) - t66515 / F::cast_from(384.0_f64) + t66518 / F::cast_from(2304.0_f64) - t45224 / F::cast_from(13824.0_f64) + t4889 * t15667 / F::cast_from(54.0_f64) - t1174 * t974 * t1196 * t55677 / F::cast_from(288.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t53360;
    t66528
}
