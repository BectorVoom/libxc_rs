//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 702/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk702(t6674: f64, t743: f64, t115: f64, t2139: f64, t757: f64, t188: f64, t1917: f64, t732: f64, t1916: f64, t1955: f64, t1912: f64, t2048: f64, t559: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6675 = t743 * t6674;
    let t6680 = t2139 * t115;
    let t6681 = t6680 * t757;
    let t6682 = t188 * t6681;
    let t6684 = t732 * t1917;
    let t6686 = t1916 * t1955;
    let t6687 = t188 * t6686;
    let t6689 = t732 * t1912;
    let t6693 = t2048 * t559;
    (t6675, t6680, t6681, t6682, t6684, t6686, t6687, t6689, t6693)
}
