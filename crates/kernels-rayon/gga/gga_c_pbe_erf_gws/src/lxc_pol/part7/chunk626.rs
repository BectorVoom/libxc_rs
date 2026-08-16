//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 626/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk626(t713: f64, t762: f64, t1597: f64, t1917: f64, t528: f64, t1413: f64, t1697: f64, t617: f64, t1809: f64, t1620: f64, t1698: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4872 = 0.66490888888888888888e-1_f64 * t762 * t713;
    let t4873 = t1597 * t713;
    let t4876 = 0.9973633333333333333e-1_f64 * t528 * t1917;
    let t4878 = t617 * t1697 * t1413;
    let t4879 = t1809 * t4878;
    let t4881 = 16.0_f64 / 15.0_f64 * t1620 * t4879;
    let t4882 = t1698 * t661;
    (t4872, t4873, t4876, t4878, t4879, t4881, t4882)
}
