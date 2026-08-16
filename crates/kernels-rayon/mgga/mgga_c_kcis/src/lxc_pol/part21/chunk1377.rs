//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1377/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1377(t96427: f64, t27077: f64, t7791: f64, t93211: f64, t93216: f64, t96404: f64, t96407: f64, t96410: f64, t96412: f64, t96420: f64, t96430: f64, t96433: f64, t97056: f64, t97267: f64) -> f64 {
    let t97465 = 0.23214722222222222222e-2_f64 * t96427;
    let t97470 = -0.23168402777777777778e-3_f64 * t97267 * t7791 - 0.92835860883789062501e-5_f64 * t27077 * t97056 + 0.23214722222222222222e-2_f64 * t96404 - 0.23214722222222222222e-2_f64 * t96407 + 0.15476481481481481481e-2_f64 * t96410 - 0.25794135802469135802e-3_f64 * t96412 - 0.15476481481481481481e-2_f64 * t96420 + t97465 - 0.92858888888888888886e-2_f64 * t96430 + 0.17024129629629629629e-1_f64 * t96433 + 0.11607361111111111111e-2_f64 * t93211 - 0.61905925925925925926e-2_f64 * t93216;
    t97470
}
