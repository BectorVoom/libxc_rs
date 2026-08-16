//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1263/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1263(t95361: f64, t95364: f64, t95366: f64, t95368: f64, t95370: f64, t95372: f64, t95374: f64, t95377: f64, t95379: f64, t95382: f64, t95384: f64, t8069: f64, t92486: f64) -> (f64, f64) {
    let t95386 = 19.0_f64 / 72.0_f64 * t95361 - t95364 / 16.0_f64 - t95366 / 288.0_f64 + t95368 / 9.0_f64 + t95370 / 9.0_f64 - 19.0_f64 / 54.0_f64 * t95372 + t95374 / 24.0_f64 - 2.0_f64 / 9.0_f64 * t95377 + 2.0_f64 / 27.0_f64 * t95379 + t95382 / 48.0_f64 - t95384 / 72.0_f64;
    let t95389 = t92486 * t8069;
    (t95386, t95389)
}
