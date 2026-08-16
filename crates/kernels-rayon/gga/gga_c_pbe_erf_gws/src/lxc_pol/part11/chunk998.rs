//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 998/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk998(t20490: f64, t3912: f64, t20281: f64, t11413: f64, t4413: f64, t3802: f64, t6469: f64, t11629: f64, t6183: f64, t11786: f64, t3783: f64, t6616: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37965 = t3912 * t20490;
    let t37994 = t3912 * t20281;
    let t37997 = t4413 * t11413;
    let t38036 = t6469 * t3802;
    let t38063 = t6183 * t11629;
    let t38143 = t6183 * t11786;
    let t38234 = t3783 * t6616;
    (t37965, t37994, t37997, t38036, t38063, t38143, t38234)
}
