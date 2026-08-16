//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 987/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk987(t1472: f64, t168: f64, t3609: f64, t1383: f64, t3380: f64, t11159: f64, t700: f64, t10033: f64, t242: f64, t1365: f64, t153: f64, t3373: f64) -> (f64, f64, f64, f64, f64) {
    let t34334 = t168 * t1472 * t3609;
    let t34336 = t3380 * t1383;
    let t34340 = t11159 * t700;
    let t34360 = t10033 * t242;
    let t34371 = t153 * t1365 * t3373;
    (t34334, t34336, t34340, t34360, t34371)
}
