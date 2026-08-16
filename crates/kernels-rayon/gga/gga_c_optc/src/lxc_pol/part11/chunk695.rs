//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 695/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk695(t146: f64, t2111: f64, t622: f64, t155: f64, t158: f64, t2078: f64, t147: f64, t2002: f64, t136: f64, t162: f64, t6165: f64, t159: f64) -> (f64, f64, f64, f64) {
    let t7037 = t146 * t2111 * t622;
    let t7061 = t155 * t158 * t2078;
    let t7073 = t146 * t147 * t2002;
    let t7089 = t6165 * t136 * t162;
    let t7091 = 0.13322897401211865505e1_f64 * t159 * t7089;
    (t7037, t7061, t7073, t7091)
}
