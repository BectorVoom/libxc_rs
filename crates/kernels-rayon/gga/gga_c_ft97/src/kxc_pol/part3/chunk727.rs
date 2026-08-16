//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 727/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk727(t14635: f64, t1882: f64, t4041: f64, t4034: f64, t4053: f64, t4057: f64, t681: f64, t89: f64, t10400: f64, t10279: f64, t1186: f64, t9733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14636 = t14635 / 27.0_f64;
    let t14637 = t1882 * t4041;
    let t14638 = 2.0_f64 / 27.0_f64 * t14637;
    let t14639 = t1882 * t4034;
    let t14640 = 2.0_f64 / 81.0_f64 * t14639;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / 27.0_f64;
    let t14683 = t89 * t681 * t4057;
    let t14684 = 2.0_f64 / 9.0_f64 * t14683;
    let t14708 = 4.0_f64 / 27.0_f64 * t10400;
    let t14711 = 4.0_f64 / 81.0_f64 * t10279;
    let t14715 = t89 * t9733 * t1186;
    (t14636, t14637, t14638, t14639, t14640, t14657, t14658, t14683, t14684, t14708, t14711, t14715)
}
