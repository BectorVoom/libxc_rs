//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1315/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1315(t297: f64, t3857: f64, t46: f64, t300: f64, t3881: f64, t1227: f64, t919: f64, t11153: f64, t179: f64, t2405: f64, t404: f64, t10253: f64, t10258: f64, t11338: f64, t11445: f64, t19140: f64, t2371: f64, t2396: f64, t27020: f64, t27044: f64, t27234: f64, t28023: f64, t28147: f64, t3061: f64, t31782: f64, t31790: f64, t3185: f64, t3206: f64, t3232: f64, t3235: f64, t3238: f64, t3898: f64, t6518: f64, t6526: f64, t758: f64, t824: f64, t8254: f64, t8428: f64, t8435: f64, t8450: f64, t934: f64) -> (f64, f64) {
    let t31897 = t3857 * t297 * t46;
    let t31919 = t300 * t3881;
    let t31920 = t1227 * t919;
    let t31936 = t404 * t179 * t2405 * t11153;
    let t31940 = 0.15244095330869239812e-2_f64 * t27234 - 0.20579528696673473746e-1_f64 * t10258 * t10253 + 0.13033701507893200039e0_f64 * t3232 * t31897 * t3238 + 0.25724410870841842184e-1_f64 * t3235 * t758 * t19140 * t11445 * t824 - 0.28582678745379824648e-3_f64 * t28023 - 0.12862205435420921092e-2_f64 * t8450 * t27044 * t28147 * t3898 - 0.38586616306262763276e-2_f64 * t3206 * t31782 * t2396 * t3061 + 0.25724410870841842184e-2_f64 * t3206 * t8254 * t2396 * t31790 + 0.38586616306262763276e-2_f64 * t8428 * t31919 * t6518 * t31920 - 0.38586616306262763276e-2_f64 * t8435 * t31919 * t6526 * t31920 - 0.25724410870841842184e-2_f64 * t3185 * t27020 * t2371 * t1227 * t824 - 0.28582678745379824648e-3_f64 * t31936 + 0.10620053080505570402e0_f64 * t11338 * t934;
    (t31920, t31940)
}
