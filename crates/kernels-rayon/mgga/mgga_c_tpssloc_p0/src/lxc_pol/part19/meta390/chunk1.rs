//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1467/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1467(t1174: f64, t11760: f64, t135: f64, t11147: f64, t3439: f64, t11719: f64, t11724: f64, t11728: f64, t11734: f64, t11770: f64, t11814: f64, t1214: f64, t1216: f64, t1227: f64, t1230: f64, t1232: f64, t15620: f64, t248: f64, t3496: f64, t3506: f64, t3508: f64, t3511: f64, t3515: f64, t3518: f64, t39097: f64, t43757: f64, t44668: f64, t44873: f64, t44879: f64, t44886: f64, t44890: f64, t44894: f64, t44896: f64, t44904: f64, t44906: f64, t44918: f64, t44929: f64, t44932: f64, t4582: f64, t974: f64) -> f64 {
    let t44936 = t1174 * t135 * t11760;
    let t44938 = t3439 * t11147;
    let t44943 = -t3515 * t4582 * t44879 * t1216 / 768.0_f64 - t44886 / 2304.0_f64 - t44890 / 1152.0_f64 + t44894 / 2304.0_f64 + t44896 * t11724 / 128.0_f64 - t1227 * t248 * t1230 * t43757 / 768.0_f64 + t44904 / 192.0_f64 + 3.0_f64 / 256.0_f64 * t11719 * t4582 * t44873 * t44906 - t11734 * t11770 / 256.0_f64 + t3506 * t248 * t1214 * t44668 * t3508 / 512.0_f64 - t44918 * t1232 / 1152.0_f64 - 3.0_f64 / 256.0_f64 * t11728 * t4582 * t44873 * t15620 + t11814 * t3496 / 512.0_f64 + t44929 * t3511 / 256.0_f64 - t44932 * t3518 / 512.0_f64 + t44936 / 27.0_f64 + t1174 * t974 * t44938 * t39097 / 6.0_f64;
    t44943
}
