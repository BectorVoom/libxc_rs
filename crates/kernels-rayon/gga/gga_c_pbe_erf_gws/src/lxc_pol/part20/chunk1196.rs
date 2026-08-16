//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1196/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1196(t15185: f64, t15285: f64, t15330: f64, t15380: f64, t1167: f64, t14821: f64, t14153: f64, t3931: f64, t3928: f64, t4063: f64, t360: f64, t898: f64) -> (f64, f64, f64, f64, f64) {
    let t15382 = t15185 + t15285 + t15330 + t15380;
    let t15386 = t14821 * t1167;
    let t15389 = t14153 * t3931;
    let t15392 = t4063 * t3928;
    let t15636 = t898 * t360;
    (t15382, t15386, t15389, t15392, t15636)
}
