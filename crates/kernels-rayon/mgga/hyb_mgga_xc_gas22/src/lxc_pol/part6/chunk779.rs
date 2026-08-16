//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 779/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk779(t222: f64, t37: f64, t4234: f64, t2455: f64, t3461: f64, t361: f64, t1410: f64, t3477: f64, t1409: f64, t968: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4236 = t222 * t37 * t4234;
    let t4238 = t2455 - 0.35616666666666666666e-1_f64 * t3461 + 0.53425e-1_f64 * t4236;
    let t4240 = 0.621814e-1_f64 * t4238 * t361;
    let t4242 = 2.0_f64 * t3477 * t1410;
    let t4243 = t1409 * t1409;
    let t4244 = t4243 * t968;
    (t4236, t4238, t4240, t4242, t4243, t4244)
}
