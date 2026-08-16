//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1321/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1321(t11896: f64, t4049: f64, t11475: f64, t4028: f64, t11734: f64, t4043: f64, t15255: f64, t51382: f64, t3799: f64, t4033: f64, t3867: f64, t11573: f64, t14015: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t57134 = t4049 * t11896;
    let t57138 = t4028 * t11475;
    let t57140 = t4043 * t11734;
    let t57142 = t51382 * t15255;
    let t57144 = t4033 * t3799;
    let t57146 = t4033 * t3867;
    let t57151 = t14015 * t11573;
    (t57134, t57138, t57140, t57142, t57144, t57146, t57151)
}
