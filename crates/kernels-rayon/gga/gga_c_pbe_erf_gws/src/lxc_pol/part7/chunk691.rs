//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 691/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk691(t4933: f64, t5121: f64, t5186: f64, t5319: f64, t5382: f64, t5439: f64, t5499: f64, t5566: f64, t1472: f64, t168: f64, t738: f64, t1931: f64, t703: f64) -> (f64, f64, f64) {
    let t5569 = t4933 + t5121 + t5186 + t5319 + t5382 + t5439 + t5499 + t5566;
    let t5574 = t168 * t1472 * t738;
    let t5577 = t168 * t703 * t1931;
    (t5569, t5574, t5577)
}
