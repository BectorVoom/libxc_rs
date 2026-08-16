//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1614/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1614(t1226: f64, t6169: f64, t486: f64, t6218: f64, t4978: f64, t4582: f64, t1216: f64, t17635: f64, t4987: f64, t4977: f64, t5012: f64, t11836: f64, t1218: f64, t1227: f64, t1232: f64, t15495: f64, t15727: f64, t15731: f64, t15735: f64, t15745: f64, t1737: f64, t19033: f64, t19041: f64, t19047: f64, t3506: f64, t3515: f64, t3536: f64, t4989: f64, t5024: f64, t6221: f64) -> f64 {
    let t19051 = t6169 * t1226;
    let t19056 = t486 * t6218;
    let t19057 = t19056 * t4978;
    let t19058 = t4582 * t19057;
    let t19061 = t19056 * t1216;
    let t19062 = t4582 * t19061;
    let t19067 = t4987 * t17635;
    let t19068 = t4582 * t19067;
    let t19071 = t4977 * t5012;
    let t19072 = t4582 * t19071;
    let t19075 = -19.0_f64 / 2592.0_f64 * t19033 * t1232 + t15727 / 81.0_f64 - t15731 / 6912.0_f64 + t15735 / 10368.0_f64 - t19041 / 6912.0_f64 + t3536 * t6221 / 3072.0_f64 + t19047 * t1218 / 3072.0_f64 + t15745 + t11836 / 1296.0_f64 - t19051 * t1232 / 4608.0_f64 - t15495 * t1737 / 288.0_f64 + t3506 * t19058 / 1536.0_f64 - t3515 * t19062 / 3072.0_f64 - 5.0_f64 / 1296.0_f64 * t5024 * t4989 + 5.0_f64 / 13824.0_f64 * t1227 * t19068 - t3515 * t19072 / 1536.0_f64;
    t19075
}
