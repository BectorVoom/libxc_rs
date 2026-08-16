//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1809/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1809(t25055: f64, t81591: f64, t25217: f64, t6547: f64, t25060: f64, t82209: f64, t82211: f64, t25192: f64, t81651: f64, t82074: f64, t82259: f64, t25054: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t87786 = t81591 * t25055;
    let t87796 = t6547 * t25217;
    let t87804 = t6547 * t25060;
    let t87806 = 0.25587863262083522346e0_f64 * t82209;
    let t87807 = 0.12793931631041761173e0_f64 * t82211;
    let t87835 = t81651 * t82074 * t25192;
    let t87847 = 0.12793931631041761173e0_f64 * t82259;
    let t87873 = t81651 * t82074 * t25054;
    (t87786, t87796, t87804, t87806, t87807, t87835, t87847, t87873)
}
