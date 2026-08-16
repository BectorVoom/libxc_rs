//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1228/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1228(t1905: f64, t81686: f64, t9537: f64, t2587: f64, t81151: f64, t23172: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64, t22816: f64, t23104: f64, t80967: f64) -> (f64, f64, f64, f64, f64) {
    let t81688 = t81686 * t9537 * t1905;
    let t81689 = 0.13707783890401886971e-2_f64 * t81688;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    let t81717 = 0.98696044010893586188e-1_f64 * t81716;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = 0.69792532988666768264e-2_f64 * t81735;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    (t81689, t81715, t81717, t81736, t81742)
}
