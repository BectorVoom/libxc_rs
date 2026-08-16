//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 172/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk172(t717: f64, t89: f64, t2: f64, t647: f64, t92: f64, t651: f64, t653: f64, t15: f64, t650: f64) -> (f64, f64, f64, f64) {
    let t718 = t89 * t717;
    let t720 = t647 * t92 * t2;
    let t725 = -0.66066666666666666667e-2_f64 * t651 - 0.41275e-2_f64 * t653;
    let t728 = -t720 * t650 / 12.0_f64 + t15 * t725 / 2.0_f64;
    (t718, t720, t725, t728)
}
