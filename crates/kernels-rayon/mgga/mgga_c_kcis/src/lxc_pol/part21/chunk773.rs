//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 773/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk773(t888: f64, t8920: f64, t2429: f64, t2485: f64, t2528: f64, t2720: f64, t2725: f64, t2729: f64, t2752: f64, t8526: f64, t8533: f64, t8541: f64, t874: f64, t8753: f64, t8757: f64, t8759: f64, t8765: f64) -> (f64, f64, f64) {
    let t8921 = t8920 * t888;
    let t8924 = t2429 * t2485;
    let t8926 = t2429 * t2528;
    let t8930 = 0.2671335375e-1_f64 * t2725 * t8526 + 0.200175e0_f64 * t874 * t8526 + 0.41786499999999999999e-1_f64 * t8533 - 0.41786499999999999999e-1_f64 * t8541 - 0.69644166666666666665e-2_f64 * t8753 - 0.2089325e-1_f64 * t8757 + 0.2671335375e-1_f64 * t8759 * t2729 - 0.13345e0_f64 * t874 * t8765 - 0.66725e-1_f64 * t874 * t8921 + 0.55715333333333333331e-1_f64 * t8924 + 0.27857666666666666666e-1_f64 * t8926 - 0.200175e0_f64 * t2720 * t2752;
    (t8924, t8926, t8930)
}
