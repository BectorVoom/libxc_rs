//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1257/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1257(t55140: f64, t829: f64, t830: f64, t4083: f64, t8746: f64, t2416: f64, t4227: f64, t353: f64, t859: f64, t938: f64, t53424: f64, t27047: f64, t4216: f64, t9296: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55142 = t829 * t830 * t55140;
    let t55145 = t8746 * t4083;
    let t55151 = t2416 * t4227;
    let t55154 = t859 * t353 * t55151 * t938;
    let t55161 = 35.0_f64 / 288.0_f64 * t53424;
    let t55182 = t27047 * t9296 * t4216 * t938;
    (t55142, t55145, t55151, t55154, t55161, t55182)
}
