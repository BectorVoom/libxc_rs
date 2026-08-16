//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 858/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk858(t713: f64, t722: f64, t9203: f64, t730: f64, t2860: f64, t2866: f64, t1987: f64, t3622: f64, t2751: f64, t7483: f64, t2787: f64, t7411: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9205 = t713 * t9203 * t722;
    let t9207 = 0.5848223622634646207e0_f64 * t730 * t9205;
    let t9209 = 0.23392894490538584828e1_f64 * t2860 * t2866;
    let t9211 = 0.5848223622634646207e0_f64 * t1987 * t3622;
    let t9213 = 4.0_f64 * t7483 * t2751;
    let t9215 = 0.32163958997385070134e2_f64 * t7411 * t2787;
    (t9205, t9207, t9209, t9211, t9213, t9215)
}
