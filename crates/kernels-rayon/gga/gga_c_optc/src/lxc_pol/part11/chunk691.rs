//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 691/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk691(t136: f64, t2079: f64, t634: f64, t108: f64, t6567: f64, t117: f64, t56: f64, t104: f64, t137: f64, t131: f64, t6165: f64, t130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6892 = t2079 * t136;
    let t6893 = t634 * t6892;
    let t6896 = t108 * t6567;
    let t6899 = 455.0_f64 / 1296.0_f64 * t6896 * t56 * t117;
    let t6915 = t137 * t104;
    let t6916 = 1.0_f64 / t6915;
    let t6917 = t136 * t6916;
    let t6922 = t131 * t6165;
    let t6923 = t130 * t6922;
    (t6892, t6893, t6896, t6899, t6916, t6917, t6922, t6923)
}
