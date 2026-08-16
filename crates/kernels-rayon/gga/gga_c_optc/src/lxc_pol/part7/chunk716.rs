//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 716/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk716(t601: f64, t6825: f64, t6735: f64, t87: f64, t40: f64, t1906: f64, t591: f64, t2045: f64, t559: f64, t1979: f64, t1983: f64, t518: f64, t622: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6827 = 0.35089340384731224426e1_f64 * t601 * t6825;
    let t6828 = t6735 * t87;
    let t6829 = t40 * t6828;
    let t6830 = t1906 * t591;
    let t6831 = t40 * t6830;
    let t6832 = 3.0_f64 * t6831;
    let t6833 = t2045 * t559;
    let t6834 = 36.0_f64 * t6833;
    let t6835 = t1979 * t1983;
    let t6836 = 0.73246220147012639764e-3_f64 * t6835;
    let t6838 = t518 * t622 * t84;
    (t6827, t6828, t6829, t6830, t6832, t6834, t6836, t6838)
}
