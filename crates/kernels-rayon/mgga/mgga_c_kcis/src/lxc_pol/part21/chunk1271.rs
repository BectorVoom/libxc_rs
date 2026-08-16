//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1271/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1271(t3178: f64, t5096: f64, t14739: f64, t26930: f64, t1169: f64, t376: f64, t14650: f64, t95453: f64, t95455: f64, t95457: f64, t95459: f64, t95461: f64, t95464: f64, t95466: f64, t95468: f64) -> (f64, f64, f64, f64) {
    let t95470 = t3178 * t5096;
    let t95472 = t26930 * t14739;
    let t95474 = t1169 * t376;
    let t95475 = t95474 * t14650;
    let t95477 = 11.0_f64 / 27.0_f64 * t95453 - 3.0_f64 / 8.0_f64 * t95455 + t95457 / 288.0_f64 - t95459 / 24.0_f64 - t95461 / 12.0_f64 + t95464 / 6.0_f64 + t95466 / 64.0_f64 - t95468 / 128.0_f64 - t95470 / 12.0_f64 + t95472 / 48.0_f64 + t95475 / 36.0_f64;
    (t95470, t95472, t95475, t95477)
}
