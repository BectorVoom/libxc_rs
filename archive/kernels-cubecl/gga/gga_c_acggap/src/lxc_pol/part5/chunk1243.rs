//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1243/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1243<F: Float>(t13293: F, t13299: F, t6403: F, t8401: F, t1095: F, t1795: F, t384: F, t398: F, t879: F, t1017: F, t1180: F, t1181: F, t13286: F, t13287: F, t1532: F, t15386: F, t17445: F, t17450: F, t17454: F, t17468: F, t17480: F, t175: F, t1753: F, t17656: F, t1854: F, t20124: F, t3196: F, t3403: F, t397: F, t4313: F, t5984: F, t922: F) -> F {
    let t22809 = t13293 * t13299 * t8401 * t6403;
    let t22818 = t384 * t398 * t1095 * t1795 * t879;
    let t22826 = -F::cast_from(0.51448821741683684367e-2_f64) * t1180 * t1181 * t4313 * t1753 * t1017 + F::cast_from(0.17149607247227894789e-2_f64) * t17445 + F::cast_from(0.85748036236139473944e-3_f64) * t17450 - F::cast_from(0.17149607247227894789e-2_f64) * t17454 - F::cast_from(0.17149607247227894789e-1_f64) * t3403 * t1181 * t1532 * t1753 * t922 - F::cast_from(0.51448821741683684366e-2_f64) * t17656 * t15386 * t8401 * t5984 - F::cast_from(0.17149607247227894789e-2_f64) * t22809 - F::cast_from(0.13719685797782315831e-1_f64) * t13286 * t13287 * t1854 * t3196 + F::cast_from(0.42874018118069736972e-3_f64) * t22818 + F::cast_from(0.34299214494455789578e-2_f64) * t17468 - F::cast_from(0.68598428988911579156e-2_f64) * t17480 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t398 * t175 * t20124;
    t22826
}
