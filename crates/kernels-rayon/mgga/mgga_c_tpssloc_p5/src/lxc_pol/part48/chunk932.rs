//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 932/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk932(t114121: f64, t1351: f64, t1992: f64, t550: f64, t6955: f64, t6976: f64, t31091: f64, t80650: f64, t22633: f64, t31100: f64, t1985: f64, t22666: f64, t31123: f64) -> (f64, f64, f64, f64, f64) {
    let t114122 = 0.16449340668482264365e-1_f64 * t114121;
    let t114127 = 0.3289868133696452873e-1_f64 * t1992 * t6976 * t6955 * t1351 * t550;
    let t114140 = 0.6579736267392905746e-1_f64 * t1992 * t80650 * t31091;
    let t114145 = 0.6579736267392905746e-1_f64 * t22633 * t80650 * t31100;
    let t114150 = 0.3289868133696452873e-1_f64 * t1985 * t22666 * t31123;
    (t114122, t114127, t114140, t114145, t114150)
}
