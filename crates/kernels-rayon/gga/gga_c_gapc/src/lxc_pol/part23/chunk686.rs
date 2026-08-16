//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 686/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk686(t2211: f64, t786: f64, t2492: f64, t923: f64, t1971: f64, t327: f64, t2598: f64, t875: f64, t1: f64, t350: f64, t818: f64, t3787: f64) -> (f64, f64, f64, f64, f64) {
    let t7453 = t2211 * t786;
    let t7460 = t2492 * t923;
    let t7502 = t1971 * t327;
    let t7503 = t2598 * t875;
    let t7510 = t818 * t1 * t350;
    let t7511 = t3787 * t7510;
    (t7453, t7460, t7502, t7503, t7511)
}
