//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1251/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1251(t49847: f64, t6241: f64, t6240: f64, t858: f64, t867: f64, t3128: f64, t44315: f64, t11592: f64, t13491: f64, t2121: f64, t337: f64, t3772: f64, t3791: f64, t9119: f64) -> (f64, f64, f64, f64, f64) {
    let t49853 = t49847 * t6241;
    let t49857 = 3.0_f64 / 8.0_f64 * t6240 * t867 * t858 * t49853;
    let t49859 = t3128 * t44315 / 12.0_f64;
    let t49861 = t11592 * t13491 / 32.0_f64;
    let t49875 = t9119 * t2121 * t337 * t3791 * t3772 / 16.0_f64;
    (t49853, t49857, t49859, t49861, t49875)
}
