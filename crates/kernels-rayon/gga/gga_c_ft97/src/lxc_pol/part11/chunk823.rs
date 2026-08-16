//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 823/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk823(t848: f64, t871: f64, t10478: f64, t319: f64, t2766: f64, t10491: f64, t2843: f64, t863: f64, t2681: f64, t309: f64, t10580: f64, t312: f64, t9570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15254 = t848 * t871;
    let t15290 = t10478 * t319;
    let t15294 = t2766 * t871;
    let t15299 = t10491 * t319;
    let t15312 = t848 * t2843;
    let t15365 = t2766 * t863;
    let t15369 = t2681 * t309;
    let t15385 = t10580 * t309;
    let t15386 = t312 * t9570;
    (t15254, t15290, t15294, t15299, t15312, t15365, t15369, t15385, t15386)
}
