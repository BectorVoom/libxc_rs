//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 885/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk885(t159: f64, t9729: f64, t2461: f64, t730: f64, t167: f64, t2478: f64, t164: f64, t2475: f64, t2479: f64, t9689: f64, t9692: f64, t9695: f64, t9698: f64, t9702: f64, t9704: f64, t9706: f64, t9709: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = 1.0_f64 / t2478 / t167;
    let t9734 = t9731 * t9733;
    let t9738 = 1.0_f64 / t2475 / t164;
    let t9739 = t159 * t9738;
    let t9740 = t9731 * t2479;
    let t9751 = -0.47063e1_f64 * t9689 + 0.31375333333333333334e1_f64 * t9692 - 0.36604555555555555556e1_f64 * t9695 - 0.16068111111111111111e1_f64 * t9698 + 0.28051666666666666666e0_f64 * t9702 - 0.56103333333333333332e0_f64 * t9704 - 0.6545388888888888889e0_f64 * t9706 - 0.46308888888888888888e0_f64 * t9709;
    (t9730, t9731, t9734, t9739, t9740, t9751)
}
