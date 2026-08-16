//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 939/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk939(t6588: f64, t899: f64, t900: f64, t6593: f64, t855: f64, t859: f64, t6238: f64, t837: f64, t863: f64, t6045: f64, t864: f64, t1477: f64, t2153: f64) -> (f64, f64, f64, f64, f64) {
    let t21117 = t899 * t900 * t6588;
    let t21121 = t855 * t6593 * t859;
    let t21245 = t863 * t6238 * t837;
    let t21253 = t863 * t864 * t6045;
    let t21293 = t863 * t2153 * t1477;
    (t21117, t21121, t21245, t21253, t21293)
}
