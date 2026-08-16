//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 701/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk701(t1466: f64, t448: f64, t292: f64, t4: f64, t13: f64, t14: f64, t1425: f64, t440: f64, t1424: f64, t1431: f64, t8: f64, t82: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4779 = t448 * t1466;
    let t4783 = 1.0_f64 / t4 / t292;
    let t4784 = sigma0 * t4783;
    let t4793 = t14 * t13;
    let t4794 = 1.0_f64 / t4793;
    let t4795 = t1425 * t440;
    let t4796 = t4794 * t4795;
    let t4799 = t1424 * t440;
    let t4800 = t4799 * t1431;
    let t4803 = t8 * t82;
    (t4779, t4784, t4794, t4795, t4796, t4800, t4803)
}
