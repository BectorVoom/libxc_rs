//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 276/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk276(t1688: f64, t300: f64, t1147: f64, t1156: f64, t1694: f64, t1164: f64, t1420: f64, t338: f64, t1178: f64, t1409: f64, t1177: f64, t1111: f64, t1668: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1701 = 0.19751673498613801407e-1_f64 * t300 * t1688;
    let t1703 = t1147 * t1694 * t1156;
    let t1705 = 0.5848223622634646207e0_f64 * t1164 * t1703;
    let t1706 = t1420 * t338;
    let t1709 = t1178 * t1409;
    let t1710 = t1177 * t1709;
    let t1714 = t1111 / 6.0_f64 - t1668 / 6.0_f64;
    (t1701, t1703, t1705, t1706, t1709, t1710, t1714)
}
