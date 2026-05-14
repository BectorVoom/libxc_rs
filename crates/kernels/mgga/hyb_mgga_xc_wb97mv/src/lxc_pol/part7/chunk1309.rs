//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1309/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1309<F: Float>(t4319: F, t7366: F, t11323: F, t2469: F, t11332: F, t260: F, t2475: F, t4293: F, t7376: F, t9400: F, t9595: F, t2517: F, t1003: F, t1005: F, t11541: F, t1436: F, t2594: F, t2597: F, t2617: F, t27062: F, t27346: F, t27359: F, t31726: F, t31730: F, t3600: F, t3608: F, t4394: F, t7434: F, t9301: F, t9514: F) -> (F, F, F, F, F, F) {
    let t31944 = 1.0 * t7366 * t4319;
    let t31946 = 2.0 * t2469 * t11323;
    let t31955 = t260 * t11332;
    let t31960 = 24.0 * t7376 * t4293 * t2475;
    let t31962 = 12.0 * t9400 * t9595;
    let t31965 = 6.0 * t2517 * t4319 * t2475;
    let t31976 = t31944 + t31946 - 0.41016075432865626631e4 * t27359 * t9514 * t31726 + 0.4155806185363551302e3 * t27062 * t3600 * t31726 - 0.17315859105681463759e2 * t7434 * t4394 - 0.11696447245269292414e1 * t31955 * t1005 - t31960 + t31962 + t31965 - 0.17315859105681463759e2 * t11541 * t2617 - 0.11696447245269292414e1 * t27346 * t1436 - 0.34631718211362927518e2 * t1003 * t2594 * t31730 * t2597 - 0.69263436422725855034e2 * t3608 * t9301;
    (t31944, t31946, t31960, t31962, t31965, t31976)
}
