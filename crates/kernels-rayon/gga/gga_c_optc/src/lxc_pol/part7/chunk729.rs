//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 729/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk729(t6900: f64, t6959: f64, t2096: f64, t669: f64, t2105: f64, t664: f64, t668: f64, t145: f64, t2107: f64, t708: f64, t2189: f64, t2126: f64, t6786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6960 = t6900 + t6959;
    let t6964 = t2096 * t669;
    let t6968 = t664 * t2105;
    let t6975 = t668 * t668;
    let t6976 = 1.0_f64 / t6975;
    let t6977 = t145 * t6976;
    let t6978 = t2107 * t708;
    let t6982 = t2105 * t708;
    let t6983 = t6982 * t2189;
    let t6986 = t2126 * t6786;
    (t6960, t6964, t6968, t6975, t6976, t6977, t6978, t6982, t6983, t6986)
}
