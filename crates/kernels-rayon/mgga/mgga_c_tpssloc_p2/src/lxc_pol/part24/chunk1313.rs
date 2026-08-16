//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1313/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1313(t131: f64, t2587: f64, t81142: f64, t1905: f64, t9537: f64, t23004: f64, t23110: f64, t23185: f64, t22987: f64, t25038: f64, t25248: f64, t2553: f64) -> (f64, f64, f64, f64) {
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = 0.13707783890401886971e-2_f64 * t81688;
    let t81691 = t23185 * t23110 * t23004;
    let t81695 = t25038 * t25248 * t22987 * t2553;
    (t81686, t81689, t81691, t81695)
}
