//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 823/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk823(t34: f64, t597: f64, t1033: f64, t1683: f64, t2749: f64, t633: f64, t219: f64, t641: f64, t1639: f64, t5219: f64, t995: f64, t5212: f64, t626: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7468 = t597 * t34;
    let t7474 = 8.0_f64 / 45.0_f64 * t1033 * t1683;
    let t7478 = 8.0_f64 / 45.0_f64 * t633 * t2749;
    let t7483 = t641 * t219;
    let t7490 = t1639 * t219;
    let t7495 = t5219 * t995;
    let t7499 = t5212 * t626;
    (t7468, t7474, t7478, t7483, t7490, t7495, t7499)
}
