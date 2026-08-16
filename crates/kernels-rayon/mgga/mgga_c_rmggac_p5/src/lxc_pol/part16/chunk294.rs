//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 294/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk294(t537: f64, t809: f64, t278: f64, t815: f64, t90: f64, t541: f64, t95: f64, t547: f64, t820: f64, t101: f64, t102: f64, t544: f64, t548: f64, t832: f64, t87: f64, t98: f64, rho1: f64, tau1: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1710 = t537 * t537;
    let t1711 = t809 * t1710;
    let t1714 = t278 + t815;
    let t1715 = t90 * t1714;
    let t1718 = t541 * rho1;
    let t1720 = 1.0_f64 / t95 / t1718;
    let t1721 = tau1 * t1720;
    let t1726 = t547 * t547;
    let t1727 = t820 * t1726;
    let t1730 = -t1714;
    let t1731 = t101 * t1730;
    let t1734 = 20.0_f64 / 9.0_f64 * t87 * t1711 + 10.0_f64 / 3.0_f64 * t87 * t1715 + 80.0_f64 / 9.0_f64 * t1721 * t102 - 100.0_f64 / 9.0_f64 * t544 * t548 + 20.0_f64 / 9.0_f64 * t98 * t1727 + 10.0_f64 / 3.0_f64 * t98 * t1731 - t832;
    (t1710, t1711, t1714, t1715, t1721, t1726, t1730, t1734)
}
