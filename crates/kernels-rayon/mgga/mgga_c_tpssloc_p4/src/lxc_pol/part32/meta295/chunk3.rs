//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1315/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1315(t118: f64, t168: f64, t2477: f64, t2510: f64, t725: f64, t740: f64, t9457: f64, t9476: f64, t9484: f64, t9697: f64, t9730: f64, t9734: f64, t9739: f64, t9740: f64, t9752: f64, t9755: f64, t9758: f64, t9759: f64, t9762: f64, t9763: f64, t9766: f64, t9780: f64, t9781: f64, t9789: f64, t9793: f64, t9797: f64) -> f64 {
    let t9798 = 0.2069040516770936012e4_f64 * t9730 * t9734 + t9457 - 0.19298375398431042081e3_f64 * t9739 * t9740 + 1.0_f64 * t725 * t9752 + 0.35089341735807877242e1_f64 * t2510 * t9755 - t9476 - t9484 + 0.10254018858216406658e4_f64 * t9758 * t9759 - 0.10389515463408878255e3_f64 * t9762 * t9763 + 0.5848223622634646207e0_f64 * t740 * t9766 - t9780 + 6.0_f64 * t2477 * t9781 + 0.16562821945185185185e-2_f64 * t118 * t9697 * t168 + t9789 - t9793 - t9797;
    t9798
}
