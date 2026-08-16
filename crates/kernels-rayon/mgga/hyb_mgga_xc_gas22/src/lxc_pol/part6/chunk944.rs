//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 944/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk944(t7: f64, t132: f64, t8473: f64, t8530: f64, t8553: f64, t8583: f64, t1793: f64, t5891: f64, t1370: f64, t6666: f64, t2291: f64, t6669: f64, t2311: f64, t3418: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t8586 = piecewise3(t134, 0.0_f64, t8473 + t8530 + t8553 + t8583);
    let t8587 = 2.0_f64 * t1793;
    let t8588 = 6.0_f64 * t5891;
    let t8589 = t8587 - t8588;
    let t8590 = piecewise3(t8, 0.0_f64, t8589);
    let t8599 = t6666 * t1370;
    let t8600 = t6669 * t2291;
    let t8601 = t8599 * t8600;
    let t8604 = t2311 * t3418;
    (t8586, t8587, t8588, t8589, t8590, t8600, t8601, t8604)
}
