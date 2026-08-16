//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 230/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk230(t136: f64, t652: f64, t162: f64, t159: f64, t133: f64, t155: f64, t158: f64) -> (f64, f64) {
    let t700 = t652 * t136;
    let t701 = t700 * t162;
    let t703 = 0.35266493120854938101e-1_f64 * t159 * t701;
    let t705 = t155 * t158 * t133;
    (t703, t705)
}
