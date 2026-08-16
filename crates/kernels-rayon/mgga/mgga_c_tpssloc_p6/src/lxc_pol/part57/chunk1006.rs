//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1006/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1006(t28: f64, t265: f64, t504: f64, t128193: f64, t128239: f64, t128278: f64, t1409: f64, t33547: f64, t52: f64, t5398: f64, t8591: f64, t113: f64, t128201: f64, t122617: f64, t126127: f64, t126132: f64, t127720: f64, t127722: f64, t127726: f64, t127728: f64, t127730: f64, t127736: f64, t127738: f64, t127742: f64, t1459: f64, t19451: f64, t1976: f64, t2040: f64, t24999: f64, t28943: f64, t28959: f64, t29205: f64, t33085: f64, t6517: f64, t7796: f64, t8529: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t128280 = piecewise3(t505, 0.0_f64, t128193);
    let t128287 = piecewise3(t401, t128239 + t128278, t128280 * t52 / 2.0_f64 - t33547 * t1409 - t8591 * t5398 / 2.0_f64);
    let t128289 = t113 * (t128201 + t128287);
    let t128293 = -4.0_f64 * t122617 * t1459 - 4.0_f64 * t126127 * t2040 - 2.0_f64 * t126132 * t2040 - 2.0_f64 * t19451 * t8529 - t1976 * t28943 - 2.0_f64 * t1976 * t28959 - 4.0_f64 * t24999 * t7796 - 4.0_f64 * t29205 * t6517 - 4.0_f64 * t33085 * t7796 - t127720 - t127722 - t127726 - t127728 - t127730 + t127736 - t127738 - t127742 - t128289;
    t128293
}
