//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2078/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2078(t6802: f64, t82713: f64, t3158: f64, t6796: f64, t23665: f64, t23674: f64, t23600: f64, t995: f64, t23680: f64, t23606: f64, t225: f64, t23494: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82714 = t82713 * t6802;
    let t82716 = t6796 * t3158;
    let t82717 = t82716 * t6802;
    let t82734 = t23665 * t23674;
    let t82736 = t23600 * t995;
    let t82737 = t82736 * t23680;
    let t82739 = t82736 * t23606;
    let t82750 = t23494 * t225;
    (t82714, t82716, t82717, t82734, t82736, t82737, t82739, t82750)
}
