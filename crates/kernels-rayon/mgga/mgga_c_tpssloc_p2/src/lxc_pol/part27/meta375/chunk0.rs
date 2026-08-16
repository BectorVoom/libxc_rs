//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1544/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1544(t13546: f64, t977: f64, t13555: f64, t2979: f64, t13528: f64, t13532: f64, t10214: f64, t13537: f64, t13969: f64, t4595: f64, t3130: f64, t1616: f64, t2780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14006 = t977 * t13546;
    let t14009 = t2979 * t13555;
    let t14012 = t2979 * t13528;
    let t14015 = t2979 * t13532;
    let t14018 = t10214 * t13537;
    let t14025 = t13969 * t4595;
    let t14027 = t3130 * t14025 / 1152.0_f64;
    let t14032 = t1616 * t2780;
    (t14006, t14009, t14012, t14015, t14018, t14025, t14027, t14032)
}
