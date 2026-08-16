//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1392/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1392(t11166: f64, t948: f64, t969: f64, t10876: f64, t2516: f64, t2520: f64, t4238: f64, t2524: f64, t1410: f64, t25273: f64, t3514: f64, t9099: f64) -> (f64, f64, f64, f64, f64) {
    let t30196 = t11166 * t948;
    let t30198 = 2.0_f64 * t30196 * t969;
    let t30200 = 1.0_f64 * t10876 * t2516;
    let t30201 = t4238 * t2520;
    let t30203 = 0.16081979498692535067e2_f64 * t30201 * t2524;
    let t30205 = 2.0_f64 * t25273 * t1410;
    let t30207 = 4.0_f64 * t9099 * t3514;
    (t30198, t30200, t30203, t30205, t30207)
}
