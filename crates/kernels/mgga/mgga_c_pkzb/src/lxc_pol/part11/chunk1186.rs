//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1186/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1186<F: Float>(t297: F, t3857: F, t46: F, t300: F, t3881: F, t1227: F, t919: F, t11153: F, t179: F, t2405: F, t404: F, t10253: F, t10258: F, t11338: F, t11445: F, t19140: F, t2371: F, t2396: F, t27020: F, t27044: F, t27234: F, t28023: F, t28147: F, t3061: F, t31782: F, t31790: F, t3185: F, t3206: F, t3232: F, t3235: F, t3238: F, t3898: F, t6518: F, t6526: F, t758: F, t824: F, t8254: F, t8428: F, t8435: F, t8450: F, t934: F) -> (F, F) {
    let t31897 = t3857 * t297 * t46;
    let t31919 = t300 * t3881;
    let t31920 = t1227 * t919;
    let t31936 = t404 * t179 * t2405 * t11153;
    let t31940 = 0.15244095330869239812e-2 * t27234 - 0.20579528696673473746e-1 * t10258 * t10253 + 0.13033701507893200039e0 * t3232 * t31897 * t3238 + 0.25724410870841842184e-1 * t3235 * t758 * t19140 * t11445 * t824 - 0.28582678745379824648e-3 * t28023 - 0.12862205435420921092e-2 * t8450 * t27044 * t28147 * t3898 - 0.38586616306262763276e-2 * t3206 * t31782 * t2396 * t3061 + 0.25724410870841842184e-2 * t3206 * t8254 * t2396 * t31790 + 0.38586616306262763276e-2 * t8428 * t31919 * t6518 * t31920 - 0.38586616306262763276e-2 * t8435 * t31919 * t6526 * t31920 - 0.25724410870841842184e-2 * t3185 * t27020 * t2371 * t1227 * t824 - 0.28582678745379824648e-3 * t31936 + 0.10620053080505570402e0 * t11338 * t934;
    (t31920, t31940)
}
