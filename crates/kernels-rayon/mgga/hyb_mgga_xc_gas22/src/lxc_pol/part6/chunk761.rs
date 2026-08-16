//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 761/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk761(t222: f64, t37: f64, t4104: f64, t2165: f64, t3300: f64, t251: f64, t1347: f64, t3316: f64, t1346: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4106 = t222 * t37 * t4104;
    let t4108 = t2165 - 0.35616666666666666666e-1_f64 * t3300 + 0.53425e-1_f64 * t4106;
    let t4110 = 0.621814e-1_f64 * t4108 * t251;
    let t4112 = 2.0_f64 * t3316 * t1347;
    let t4113 = t1346 * t1346;
    let t4114 = t4113 * t810;
    (t4106, t4108, t4110, t4112, t4113, t4114)
}
