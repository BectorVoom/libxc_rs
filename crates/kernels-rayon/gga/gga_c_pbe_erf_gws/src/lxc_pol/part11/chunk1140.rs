//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1140/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1140(t1006: f64, t12590: f64, t3392: f64, t3493: f64, t41595: f64, t41633: f64, t10629: f64, t3500: f64, t12616: f64, t5211: f64, t7106: f64, t41666: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48148 = 16.0_f64 / 5.0_f64 * t1006 * t12590;
    let t48150 = 16.0_f64 / 5.0_f64 * t3493 * t3392;
    let t48152 = 32.0_f64 / 15.0_f64 * t41595;
    let t48153 = 64.0_f64 / 45.0_f64 * t41633;
    let t48155 = 32.0_f64 / 15.0_f64 * t10629 * t3500;
    let t48158 = 32.0_f64 / 15.0_f64 * t5211 * t7106 * t12616;
    let t48159 = 32.0_f64 / 45.0_f64 * t41666;
    (t48148, t48150, t48152, t48153, t48155, t48158, t48159)
}
