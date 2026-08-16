//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 958/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk958(t285: f64, t545: f64, t8279: f64, t1480: f64, t551: f64, t6045: f64, t991: f64, t169: f64, t700: f64, t8361: f64, t1086: f64, t4598: f64) -> (f64, f64, f64, f64) {
    let t25965 = t8279 * t545 * t285;
    let t26012 = t6045 * t991 * t551 * t1480;
    let t26031 = t169 * t8361 * t700;
    let t26034 = t169 * t1086 * t4598;
    (t25965, t26012, t26031, t26034)
}
