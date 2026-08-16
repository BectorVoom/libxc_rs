//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 982/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk982(t3474: f64, t948: f64, t969: f64, t2516: f64, t3477: f64, t1396: f64, t2520: f64) -> (f64, f64, f64, f64) {
    let t9099 = t3474 * t948;
    let t9101 = 2.0_f64 * t9099 * t969;
    let t9103 = 1.0_f64 * t3477 * t2516;
    let t9104 = t1396 * t2520;
    (t9099, t9101, t9103, t9104)
}
