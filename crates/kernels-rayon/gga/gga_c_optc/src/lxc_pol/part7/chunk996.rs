//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 996/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk996(t1912: f64, t2229: f64, t2234: f64, t6681: f64, t732: f64, t737: f64, t188: f64, t1955: f64, t6680: f64, t1917: f64, t115: f64, t6568: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21962 = t2229 * t1912;
    let t21964 = t2234 * t1912;
    let t21968 = t732 * t6681;
    let t21970 = t737 * t6681;
    let t21973 = t188 * t6680 * t1955;
    let t21975 = t2229 * t1917;
    let t21977 = t2234 * t1917;
    let t21979 = t6568 * t115;
    (t21962, t21964, t21968, t21970, t21973, t21975, t21977, t21979)
}
