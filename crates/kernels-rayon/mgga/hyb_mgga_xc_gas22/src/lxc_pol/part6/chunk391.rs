//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 391/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk391(t132: f64, t1388: f64, t1445: f64, t340: f64, t394: f64, t295: f64, t412: f64, t420: f64, t303: f64, t209: f64, t306: f64, t211: f64, t409: f64, dens_threshold: f64, rho1: f64, tau1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t1449 = piecewise3(t134, 0.0_f64, t1388 * t394 / 2.0_f64 + t340 * t1445 / 2.0_f64);
    let t1454 = t295 * t412;
    let t1455 = 1.0_f64 / t420;
    let t1459 = t303 * tau1;
    let t1464 = t306 * t209;
    let t1467 = t409 * t211;
    (t1449, t1454, t1455, t1459, t1464, t1467)
}
