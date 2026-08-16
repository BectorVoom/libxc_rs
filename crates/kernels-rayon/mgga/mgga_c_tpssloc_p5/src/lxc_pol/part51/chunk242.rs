//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 242/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk242(t906: f64, t241: f64, t340: f64, t884: f64, t136: f64, t886: f64, t897: f64, t899: f64, t902: f64, t290: f64, t893: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t907 = 0.82156666666666666667e-1_f64 * t906;
    let t908 = t241 * t340;
    let t909 = t908 * t884;
    let t910 = t136 * t909;
    let t912 = 0.1898925e1_f64 * t897 - t899 - 0.29896666666666666667e0_f64 * t886 + 0.3071625e0_f64 * t902 - t907 - 0.82156666666666666667e-1_f64 * t910;
    let t913 = 1.0_f64 / t290;
    let t914 = t912 * t913;
    let t916 = 1.0_f64 * t893 * t914;
    let t917 = 0.17123333333333333333e-1_f64 * t880;
    (t907, t908, t909, t910, t912, t913, t914, t916, t917)
}
