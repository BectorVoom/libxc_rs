//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1393/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1393<F: Float>(t3477: F, t9168: F, t21378: F, t4244: F, t4270: F, t7065: F, t10954: F, t2473: F, t11116: F, t1422: F, t21393: F, t21396: F, t21620: F, t21633: F, t25214: F, t25217: F, t25220: F, t2538: F, t2539: F, t2554: F, t2560: F, t29757: F, t29760: F, t29788: F, t30127: F, t30182: F, t30184: F, t30194: F, t30198: F, t30200: F, t30203: F, t30205: F, t30207: F, t374: F, t4284: F, t4300: F, t7002: F, t7059: F, t9241: F) -> (F, F, F, F, F) {
    let t30209 = F::cast_from(2.0_f64) * t3477 * t9168;
    let t30211 = F::cast_from(2.0_f64) * t21378 * t4244;
    let t30213 = F::cast_from(1.0_f64) * t7065 * t4270;
    let t30215 = F::cast_from(2.0_f64) * t2473 * t10954;
    let t30216 = F::cast_from(6.0_f64) * t2560 * t4284 * t2554 + F::cast_from(0.11579025239058625248e4_f64) * t7059 * t4300 * t2539 - F::cast_from(4.0_f64) * t2538 * t1422 * t9241 - F::cast_from(0.19298375398431042081e3_f64) * t7002 * t4300 * t2554 - F::cast_from(0.24828486201251232145e5_f64) * t21633 * t11116 * t2539 - F::cast_from(0.310907e-1_f64) * (t21620 - F::cast_from(0.10654518518518518518e0_f64) * t21393 + F::cast_from(0.22831111111111111111e-1_f64) * t21396 - F::cast_from(0.10654518518518518518e0_f64) * t25214 + F::cast_from(0.91324444444444444442e-1_f64) * t25217 - F::cast_from(0.34246666666666666666e-1_f64) * t25220 + F::cast_from(0.22831111111111111111e-1_f64) * t29757 - F::cast_from(0.34246666666666666666e-1_f64) * t29760 + F::cast_from(0.5137e-1_f64) * t29788) * t374 - t30127 - t30182 - t30184 - F::cast_from(0.19751673498613801407e-1_f64) * t30194 - t30198 - t30200 - t30203 - t30205 - t30207 - t30209 + t30211 - t30213 - t30215;
    (t30209, t30211, t30213, t30215, t30216)
}
