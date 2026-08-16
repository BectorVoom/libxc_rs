//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 464/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk464(t156: f64, t974: f64, t496: f64, t501: f64, t978: f64, t395: f64, t1563: f64, t967: f64, t513: f64, t981: f64, t133: f64, t119: f64, t132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2878 = t156 * t974;
    let t2879 = t496 * t2878;
    let t2890 = t501 * t978;
    let t2891 = t2890 * t395;
    let t2893 = t1563 * t967;
    let t2902 = t981 * t513;
    let t2909 = t133 * t2878;
    let t2911 = t132 * t119;
    (t2879, t2890, t2891, t2893, t2902, t2909, t2911)
}
