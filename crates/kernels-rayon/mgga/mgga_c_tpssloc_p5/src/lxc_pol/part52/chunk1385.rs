//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1385/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1385(t31832: f64, t7754: f64, t8689: f64, t8944: f64, t26164: f64, t24994: f64, t24996: f64, t120108: f64, t120111: f64, t120114: f64, t120171: f64, t120173: f64, t120176: f64, t120177: f64, t120658: f64, t120659: f64, t120663: f64) -> f64 {
    let t123193 = t31832 * t7754;
    let t123194 = t8689 * t8944;
    let t123195 = t123194 * t26164;
    let t123198 = t8689 * t24994;
    let t123199 = t123198 * t24996;
    let t123201 = -2.0_f64 * t120108 + t123193 - t120111 - t120114 + t120171 + 2.0_f64 * t123195 + 6.0_f64 * t120173 - t120176 + t120177 + t120658 - t120659 + 6.0_f64 * t123199 + t120663;
    t123201
}
