//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1807/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1807(t82011: f64, t82039: f64, t25273: f64, t6579: f64, t244: f64, t268: f64, t6559: f64, t25250: f64, t87202: f64, t25316: f64, t82038: f64, t23110: f64, t23185: f64, t25272: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t87687 = 0.12793931631041761173e0_f64 * t82011;
    let t87708 = 0.10417915756705434098e0_f64 * t82039;
    let t87709 = t6579 * t25273;
    let t87712 = t6559 * t244 * t268;
    let t87714 = t87712 * t87202 * t25250;
    let t87718 = t82038 * t25316;
    let t87729 = t23185 * t23110 * t25272;
    (t87687, t87708, t87709, t87712, t87714, t87718, t87729)
}
