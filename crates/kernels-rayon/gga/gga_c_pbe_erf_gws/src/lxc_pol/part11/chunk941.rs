//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 941/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk941(t20270: f64, t2276: f64, t4422: f64, t885: f64, t6158: f64, t6670: f64, t6587: f64, t899: f64, t900: f64, t6045: f64, t855: f64, t863: f64) -> (f64, f64, f64, f64, f64) {
    let t21430 = t2276 * t20270;
    let t21491 = t4422 * t885;
    let t21497 = t6158 * t6670;
    let t21507 = t899 * t900 * t6587;
    let t21511 = t863 * t855 * t6045;
    (t21430, t21491, t21497, t21507, t21511)
}
