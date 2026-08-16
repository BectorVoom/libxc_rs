//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1009/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1009(t5445: f64, t8513: f64, t8514: f64, t31691: f64, t5441: f64, t115833: f64, t126065: f64, t115903: f64, t126073: f64, t5392: f64, t5389: f64, t115834: f64, t115871: f64, t115907: f64, t121094: f64, t121121: f64, t121124: f64, t126046: f64, t126062: f64, t126091: f64, t31675: f64, t31681: f64, t33560: f64, t33564: f64, t33572: f64, t7026: f64, t8512: f64) -> f64 {
    let t128337 = t8513 * t8514 * t5445;
    let t128345 = t8513 * t31691 * t5441;
    let t128352 = t115833 * t126065;
    let t128355 = t115903 * t126073;
    let t128359 = t8513 * t8514 * t5392;
    let t128363 = t8513 * t8514 * t5389;
    let t128368 = 5.0_f64 / 6.0_f64 * t31675 * t126046 + 5.0_f64 / 12.0_f64 * t31675 * t128337 - 5.0_f64 / 9.0_f64 * t126091 * t115834 - 5.0_f64 / 18.0_f64 * t8512 * t126062 - 5.0_f64 / 36.0_f64 * t8512 * t128345 + 5.0_f64 / 6.0_f64 * t121094 * t33564 - 5.0_f64 / 18.0_f64 * t33560 * t33572 - 10.0_f64 / 3.0_f64 * t115907 * t128352 + 10.0_f64 / 9.0_f64 * t31681 * t128355 + 5.0_f64 / 18.0_f64 * t7026 * t128359 - 35.0_f64 / 12.0_f64 * t115871 * t128363 + 10.0_f64 / 27.0_f64 * t121121 + 10.0_f64 / 27.0_f64 * t121124;
    t128368
}
