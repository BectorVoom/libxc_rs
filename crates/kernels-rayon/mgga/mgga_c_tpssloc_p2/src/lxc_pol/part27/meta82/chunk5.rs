//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 546/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk546(t1088: f64, t1653: f64, t123: f64, t1087: f64, t423: f64, t1086: f64) -> (f64, f64, f64, f64, f64) {
    let t1654 = t1088 * t1653;
    let t1655 = t123 * t1654;
    let t1657 = -t1087 + 0.17808333333333333333e-1_f64 * t1655;
    let t1659 = 0.621814e-1_f64 * t1657 * t423;
    let t1661 = -t1086 / 3.0_f64 + t1655 / 3.0_f64;
    (t1654, t1655, t1657, t1659, t1661)
}
