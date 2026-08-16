//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 980/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk980(t8101: f64, t3645: f64, t725: f64, t1352: f64, t2332: f64, t8107: f64, t8118: f64, t8121: f64, t10497: f64, t150: f64, t190: f64, t2109: f64, t3572: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10518 = 8.0_f64 * t8101;
    let t10520 = 2.0_f64 * t3645 * t725;
    let t10521 = t1352 * t2332;
    let t10522 = 4.0_f64 * t8107;
    let t10523 = 0.4883052614935078681e-3_f64 * t8118;
    let t10524 = 0.18311447306006545054e-3_f64 * t8121;
    let t10525 = t150 * t10497;
    let t10526 = t10525 * t190;
    let t10528 = 4.0_f64 * t3572 * t2109;
    (t10518, t10520, t10521, t10522, t10523, t10524, t10526, t10528)
}
