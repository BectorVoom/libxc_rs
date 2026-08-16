//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1130/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1130(t132: f64, t10978: f64, t11181: f64, t1019: f64, t10853: f64, t1388: f64, t1445: f64, t340: f64, t3455: f64, t3609: f64, t394: f64, t4224: f64, t4348: f64, t932: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64) {
    let t133 = t132 <= zeta_threshold;
    let t134 = rho1 <= dens_threshold || t133;
    let t11182 = t10978 + t11181;
    let t11186 = piecewise3(t134, 0.0_f64, t10853 * t394 / 2.0_f64 + t4224 * t1019 / 2.0_f64 + t3455 * t1445 + t1388 * t3609 + t932 * t4348 / 2.0_f64 + t340 * t11182 / 2.0_f64);
    (t11182, t11186)
}
