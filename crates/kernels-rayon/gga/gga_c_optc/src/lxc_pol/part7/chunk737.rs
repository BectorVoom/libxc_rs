//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 737/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk737(t146: f64, t147: f64, t2002: f64, t688: f64, t2144: f64, t2152: f64, t2089: f64, t2182: f64, t116: f64, t6856: f64, t6944: f64, t2148: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7073 = t146 * t147 * t2002;
    let t7074 = t7073 * t688;
    let t7076 = t2144 * t2152;
    let t7078 = t2182 * t2089;
    let t7083 = t6944 * t116 * t6856;
    let t7086 = t2144 * t2148;
    (t7073, t7074, t7076, t7078, t7083, t7086)
}
