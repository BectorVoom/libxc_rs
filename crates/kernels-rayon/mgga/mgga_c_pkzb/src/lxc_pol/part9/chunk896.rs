//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 896/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk896(t24: f64, t6237: f64, t6605: f64, t5113: f64, t2467: f64, t2471: f64, t422: f64, t423: f64, t960: f64, t962: f64, t330: f64, t328: f64, t1444: f64, t42: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t90 = t24 <= zeta_threshold;
    let t332 = rho1 <= dens_threshold || t90;
    let t6606 = t6237 + t6605;
    let t6613 = piecewise3(t90, 0.0_f64, t5113);
    let t6617 = piecewise3(t332, 0.0_f64, t6606 * t423 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2467 * t962 + 3.0_f64 / 2.0_f64 * t960 * t2471 + t422 * t6613 / 2.0_f64);
    let t6618 = t330 * t6617;
    let t6619 = t328 * t6618;
    let t6620 = 0.2390625e-1_f64 * t6619;
    let t6631 = t1444 * t42;
    (t6606, t6613, t6618, t6620, t6631)
}
