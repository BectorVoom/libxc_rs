//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 664/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk664(t1448: f64, t6967: f64, t1069: f64, t1617: f64, t1022: f64, t1791: f64, t1660: f64, t197: f64, t1663: f64, t108: f64, t182: f64, t267: f64) -> (f64, f64, f64, f64, f64) {
    let t6968 = t6967 * t1448;
    let t6998 = t1069 * t1617;
    let t7027 = t1791 * t1022;
    let t7048 = t1660 * t197;
    let t7049 = t7048 * t1663;
    let t7061 = t182 * t108;
    let t7062 = t7061 * t267;
    (t6968, t6998, t7027, t7049, t7062)
}
