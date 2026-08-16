//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1050/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1050(t141: f64, t2087: f64, t21868: f64, t2080: f64, t2089: f64, t654: f64, t6919: f64, t137: f64, t136: f64, t22752: f64, t6910: f64, t6941: f64) -> (f64, f64, f64, f64, f64) {
    let t22827 = t2087 * t141 * t21868;
    let t22830 = t2080 * t2089;
    let t22832 = t654 * t6919;
    let t22834 = t137 * t137;
    let t22835 = 1.0_f64 / t22834;
    let t22836 = t136 * t22835;
    let t22838 = t22836 * t141 * t22752;
    let t22841 = t6941 * t6910;
    (t22827, t22830, t22832, t22838, t22841)
}
