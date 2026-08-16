//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1235/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1235(t45140: f64, t9016: f64, t1109: f64, t3825: f64, t3855: f64, t3065: f64, t858: f64, t8978: f64, t11419: f64, t45209: f64, t44949: f64, t13440: f64, t2118: f64, t3912: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49528 = t9016 * t45140 / 4.0_f64;
    let t49529 = t3825 * t1109;
    let t49534 = t3855 * t1109;
    let t49538 = t8978 * t3065 * t858 * t49534 / 16.0_f64;
    let t49540 = t11419 * t45209 / 2.0_f64;
    let t49545 = 7.0_f64 / 12.0_f64 * t44949;
    let t49550 = t3912 * t2118 * t13440 * t860 / 24.0_f64;
    (t49528, t49529, t49534, t49538, t49540, t49545, t49550)
}
