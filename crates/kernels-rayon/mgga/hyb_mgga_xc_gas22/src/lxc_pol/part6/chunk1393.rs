//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1393/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1393(t3477: f64, t9168: f64, t21378: f64, t4244: f64, t4270: f64, t7065: f64, t10954: f64, t2473: f64, t11116: f64, t1422: f64, t21393: f64, t21396: f64, t21620: f64, t21633: f64, t25214: f64, t25217: f64, t25220: f64, t2538: f64, t2539: f64, t2554: f64, t2560: f64, t29757: f64, t29760: f64, t29788: f64, t30127: f64, t30182: f64, t30184: f64, t30194: f64, t30198: f64, t30200: f64, t30203: f64, t30205: f64, t30207: f64, t374: f64, t4284: f64, t4300: f64, t7002: f64, t7059: f64, t9241: f64) -> (f64, f64, f64, f64, f64) {
    let t30209 = 2.0_f64 * t3477 * t9168;
    let t30211 = 2.0_f64 * t21378 * t4244;
    let t30213 = 1.0_f64 * t7065 * t4270;
    let t30215 = 2.0_f64 * t2473 * t10954;
    let t30216 = 6.0_f64 * t2560 * t4284 * t2554 + 0.11579025239058625248e4_f64 * t7059 * t4300 * t2539 - 4.0_f64 * t2538 * t1422 * t9241 - 0.19298375398431042081e3_f64 * t7002 * t4300 * t2554 - 0.24828486201251232145e5_f64 * t21633 * t11116 * t2539 - 0.310907e-1_f64 * (t21620 - 0.10654518518518518518e0_f64 * t21393 + 0.22831111111111111111e-1_f64 * t21396 - 0.10654518518518518518e0_f64 * t25214 + 0.91324444444444444442e-1_f64 * t25217 - 0.34246666666666666666e-1_f64 * t25220 + 0.22831111111111111111e-1_f64 * t29757 - 0.34246666666666666666e-1_f64 * t29760 + 0.5137e-1_f64 * t29788) * t374 - t30127 - t30182 - t30184 - 0.19751673498613801407e-1_f64 * t30194 - t30198 - t30200 - t30203 - t30205 - t30207 - t30209 + t30211 - t30213 - t30215;
    (t30209, t30211, t30213, t30215, t30216)
}
