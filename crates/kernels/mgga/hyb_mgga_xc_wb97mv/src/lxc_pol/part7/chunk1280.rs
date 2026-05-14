//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1280/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1280<F: Float>(t31332: F, t31345: F, t31359: F, t31370: F, t2283: F, t4197: F, t26492: F, t26666: F, t808: F, t2261: F, t10980: F, t838: F, t11068: F, t11100: F, t1353: F, t22435: F, t2257: F, t2262: F, t2264: F, t22755: F, t2279: F, t2287: F, t2323: F, t2325: F, t26610: F, t26621: F, t26637: F, t26786: F, t26924: F, t30801: F, t30805: F, t31275: F, t31288: F, t31302: F, t31313: F, t3383: F, t3403: F, t4203: F, t4216: F, t4219: F, t6866: F, t820: F, t827: F, t828: F, t839: F, t847: F, t848: F, t9050: F, t9056: F, t9142: F, t9152: F) -> (F, F, F) {
    let t31372 = t31332 + t31345 + t31359 + t31370;
    let t31378 = t4197 * t2283;
    let t31393 = 0.2069040516770936012e4 * t26492 * t26666 * t808;
    let t31400 = t4197 * t2261;
    let t31403 = t10980 * t838;
    let t31406 = 0.8276162067083744048e4 * t26610 * t26786 * t827 - 2.0 * t22755 * t4203 + 1.0 * t6866 * t4216 + 2.0 * t2257 * t11068 + 1.0 * t820 * (t31275 + t31288 + t31302 + t31313) * t828 + 0.32163958997385070134e2 * t22435 * t4219 + 0.5848223622634646207e0 * t839 * t31372 * t847 + 1.0 * t11100 * t2279 + 0.32163958997385070134e2 * t31378 * t2287 + 2.0 * t26924 * t1353 + 4.0 * t9056 * t3403 + 2.0 * t3383 * t9050 - 0.4155806185363551302e3 * t26637 * t9152 + 0.14035736694323150897e2 * t26621 * t9142 - t31393 + 0.34631718211362927518e2 * t2323 * t30801 * t2325 - 4.0 * t2262 * t30805 * t828 - 2.0 * t31400 * t2264 + 0.11696447245269292414e1 * t31403 * t848;
    (t31372, t31393, t31406)
}
