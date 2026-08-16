//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 672/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk672(t237: f64, t3826: f64, t3802: f64, t1217: f64, t3147: f64, t2295: f64, t3806: f64, t890: f64, t898: f64, t3819: f64, t881: f64, t2317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3827 = t237 * t3826;
    let t3829 = 0.19751673498613801407e-1_f64 * t237 * t3802;
    let t3831 = 0.11696447245269292414e1_f64 * t3147 * t1217;
    let t3833 = t2295 * t3806 * t890;
    let t3835 = 0.11696447245269292414e1_f64 * t898 * t3833;
    let t3837 = t881 * t3819 * t890;
    let t3839 = 0.5848223622634646207e0_f64 * t898 * t3837;
    let t3840 = t2317 * t3806;
    (t3827, t3829, t3831, t3833, t3835, t3837, t3839, t3840)
}
