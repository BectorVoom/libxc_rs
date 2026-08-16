//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3181/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3181(t1009: f64, t18571: f64, t1011: f64, t1212: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t1218: f64, t1227: f64, t15455: f64, t15541: f64, t15545: f64, t15656: f64, t18590: f64, t18594: f64, t18955: f64, t19047: f64, t3490: f64, t3496: f64, t3511: f64, t3518: f64, t4582: f64, t4972: f64, t5005: f64, t52817: f64, t52845: f64, t52859: f64, t61798: f64) -> (f64, f64) {
    let t65955 = t18571 * t1009;
    let t65957 = t65955 * t1011 * t1212;
    let t65962 = t65253 * t3032;
    let t65963 = t65962 * t3505;
    let t65966 = t65962 * t3514;
    let t65990 = t65957 * t1218 / 1536.0_f64 + t19047 * t3496 / 3072.0_f64 + t65963 * t3511 / 1536.0_f64 - t65966 * t3518 / 3072.0_f64 + t52817 / 576.0_f64 + t52845 / 432.0_f64 - 5.0_f64 / 2592.0_f64 * t3490 * t18955 - 5.0_f64 / 2592.0_f64 * t5005 * t15455 + 5.0_f64 / 3456.0_f64 * t5005 * t15541 + 5.0_f64 / 6912.0_f64 * t5005 * t15545 + 5.0_f64 / 1152.0_f64 * t5005 * t15656 - t3490 * t18590 / 576.0_f64 - t1227 * t4582 * t4972 * t61798 / 1152.0_f64 - t3490 * t18594 / 384.0_f64 + t52859 / 1152.0_f64;
    (t65955, t65990)
}
