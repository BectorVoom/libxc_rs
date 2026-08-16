//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1243/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1243(t2721: f64, t7983: f64, t8152: f64, t8044: f64, t2670: f64, t7212: f64, t3917: f64, t7494: f64, t10838: f64, t8164: f64, t3884: f64, t7452: f64) -> (f64, f64, f64, f64, f64) {
    let t25657 = t2721 * t8152 * t7983;
    let t25660 = t2721 * t8152 * t8044;
    let t25662 = t7212 * t2670;
    let t25664 = t3917 * t25662 * t7494;
    let t25667 = t2721 * t10838 * t8164;
    let t25670 = t3884 * t25662 * t7452;
    (t25657, t25660, t25664, t25667, t25670)
}
