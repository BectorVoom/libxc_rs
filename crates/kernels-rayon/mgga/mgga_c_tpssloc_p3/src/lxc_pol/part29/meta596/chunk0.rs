//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2026/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2026(t1905: f64, t81686: f64, t9537: f64, t23004: f64, t23110: f64, t23185: f64, t23005: f64, t6579: f64, t23181: f64, t2587: f64, t81151: f64, t23172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = 0.13707783890401886971e-2_f64 * t81688;
    let t81691 = t23185 * t23110 * t23004;
    let t81697 = t6579 * t23005;
    let t81704 = t6579 * t23181;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    (t81689, t81691, t81697, t81704, t81715, t81716)
}
