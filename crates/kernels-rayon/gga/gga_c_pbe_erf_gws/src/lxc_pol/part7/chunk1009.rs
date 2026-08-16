//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1009/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1009(t1472: f64, t168: f64, t1931: f64, t153: f64, t4867: f64, t542: f64, t5569: f64, t703: f64, t5: f64, t922: f64, t270: f64, t4573: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18336 = t168 * t1472 * t1931;
    let t18339 = t153 * t542 * t4867;
    let t18342 = t168 * t703 * t5569;
    let t18344 = t5 * t922;
    let t18347 = 0.90790602394455990432e0_f64 * t168 * t18344 * t270;
    let t18349 = t153 * t4573 * t745;
    (t18336, t18339, t18342, t18344, t18347, t18349)
}
