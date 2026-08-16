//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 796/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk796(t132: f64, t1388: f64, t1445: f64, t340: f64, t394: f64, t4224: f64, t4348: f64, t1493: f64, t416: f64, t418: f64, t196: f64, t413: f64, t3955: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t4352 = piecewise3(t134, 0.0_f64, t4224 * t394 / 2.0_f64 + t1388 * t1445 + t340 * t4348 / 2.0_f64);
    let t4356 = 1.0_f64 / t1493;
    let t4361 = t418 * t416;
    let t4363 = 1.0_f64 / t196 / t4361;
    let t4368 = t413 * t413;
    let t4369 = t418 * t3955;
    (t4352, t4356, t4363, t4368, t4369)
}
