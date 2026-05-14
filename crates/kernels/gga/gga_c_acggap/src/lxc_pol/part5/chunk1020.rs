//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1020/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1020<F: Float>(t1163: F, t1165: F, t1552: F, t322: F, t6151: F, t1106: F, t1181: F, t1879: F, t3391: F, t3382: F, t5981: F, t1131: F, t1180: F, t12478: F, t13268: F, t13364: F, t1531: F, t1532: F, t15410: F, t15429: F, t15431: F, t17185: F, t1753: F, t19834: F, t301: F, t3462: F, t3539: F, t372: F, t5615: F, t5852: F, t5922: F, t8790: F) -> (F,) {
    let t20365 = t1163 * t1165 * t1552 * t6151 * t322;
    let t20379 = t3391 * t1181 * t1879 * t1106;
    let t20383 = t3382 * t5981;
    let t20385 = -0.13719685797782315831e-1 * t17185 * t13364 * t8790 * t5615 * t301 - 0.85748036236139473944e-3 * t1180 * t1165 * t1532 * t19834 - 0.17149607247227894789e-2 * t15410 + 0.17149607247227894789e-2 * t1180 * t1181 * t1552 * t1753 * t1131 - 0.17149607247227894789e-2 * t20365 - 0.34299214494455789578e-2 * t1531 * t1181 * t5922 * t3539 * t372 - 0.60023625365297631762e-2 * t12478 - 0.17149607247227894789e-2 * t3462 * t1165 * t5852 * t13268 - 0.51448821741683684368e-2 * t20379 - 0.34299214494455789578e-2 * t15429 - 0.17149607247227894789e-1 * t15431 + 0.17149607247227894789e-2 * t20383;
    (t20385,)
}
