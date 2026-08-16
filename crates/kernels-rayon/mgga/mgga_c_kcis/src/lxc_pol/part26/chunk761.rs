//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 761/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk761(t62: f64, t8750: f64, t755: f64, t752: f64, t2479: f64, t754: f64, t775: f64, t2724: f64, t870: f64, t2726: f64, t887: f64, t217: f64, t2727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8751 = t62 * t8750;
    let t8752 = t755 * t8751;
    let t8753 = t752 * t8752;
    let t8755 = t2479 * t754;
    let t8756 = t8755 * t775;
    let t8757 = t752 * t8756;
    let t8759 = t870 * t2724;
    let t8762 = t2726 * t887;
    let t8764 = 1.0_f64 / t2727 / t217;
    (t8753, t8755, t8757, t8759, t8762, t8764)
}
