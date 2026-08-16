//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1000/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1000(t132: f64, t9279: f64, t9309: f64, t1019: f64, t1388: f64, t1445: f64, t2449: f64, t2624: f64, t340: f64, t3455: f64, t3609: f64, t394: f64, t8955: f64, t932: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t9310 = t9279 + t9309;
    let t9314 = piecewise3(t134, 0.0_f64, t8955 * t394 / 2.0_f64 + t3455 * t1019 + t1388 * t2624 / 2.0_f64 + t2449 * t1445 / 2.0_f64 + t932 * t3609 + t340 * t9310 / 2.0_f64);
    (t9310, t9314)
}
