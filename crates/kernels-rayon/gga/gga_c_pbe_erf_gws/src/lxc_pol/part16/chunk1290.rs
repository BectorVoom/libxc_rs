//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1290/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1290(t2407: f64, t26623: f64, t858: f64, t2120: f64, t3195: f64, t4033: f64, t4171: f64, t51407: f64, t4049: f64, t9661: f64, t4043: f64, t9449: f64) -> (f64, f64, f64, f64, f64) {
    let t54373 = t2407 * t858 * t26623;
    let t54374 = t2120 * t54373;
    let t54377 = t4033 * t3195;
    let t54381 = t51407 * t4171;
    let t54384 = t4049 * t9661;
    let t54386 = t4043 * t9449;
    (t54374, t54377, t54381, t54384, t54386)
}
