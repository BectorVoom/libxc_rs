//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1025/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1025(t12630: f64, t1820: f64, t5125: f64, t12650: f64, t5018: f64, t587: f64, t10924: f64, t2612: f64, t10629: f64, t2640: f64, t12821: f64, t16797: f64, t639: f64) -> (f64, f64, f64, f64, f64) {
    let t42175 = t1820 * t5125 * t12630;
    let t42187 = t587 * t5018 * t12650;
    let t42189 = t2612 * t10924;
    let t42191 = t10629 * t2640;
    let t42204 = t639 * t16797 * t12821;
    (t42175, t42187, t42189, t42191, t42204)
}
