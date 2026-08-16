//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1194/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1194(t15437: f64, t15503: f64, t15525: f64, t15565: f64, t1167: f64, t15101: f64, t14368: f64, t3931: f64, t3928: f64, t4120: f64, t360: f64, t898: f64) -> (f64, f64, f64, f64, f64) {
    let t15567 = t15437 + t15503 + t15525 + t15565;
    let t15571 = t15101 * t1167;
    let t15574 = t14368 * t3931;
    let t15577 = t4120 * t3928;
    let t15636 = t898 * t360;
    (t15567, t15571, t15574, t15577, t15636)
}
