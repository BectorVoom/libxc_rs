//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 981/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk981(t4551: f64, t713: f64, t1457: f64, t1917: f64, t762: f64, t16487: f64, t16490: f64, t16503: f64, t16508: f64, t16512: f64, t16515: f64, t18146: f64, t18149: f64) -> f64 {
    let t18150 = t4551 * t713;
    let t18152 = t1457 * t713;
    let t18155 = 0.26596355555555555555e0_f64 * t762 * t1917;
    let t18156 = 0.39894533333333333332e0_f64 * t18146 + t18149 + 0.19947266666666666666e0_f64 * t18150 - 0.26596355555555555555e0_f64 * t18152 - t18155 - t16487 - t16490 - t16503 + t16508 + t16512 - t16515;
    t18156
}
