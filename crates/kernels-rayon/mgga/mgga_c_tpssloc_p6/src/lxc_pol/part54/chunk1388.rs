//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1388/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1388(t118588: f64, t118596: f64, t118602: f64, t112818: f64, t112821: f64, t112830: f64, t112847: f64, t114732: f64, t114734: f64, t114737: f64, t114739: f64, t118586: f64, t118590: f64, t118592: f64, t118594: f64, t118606: f64, t118608: f64, t118610: f64, t118612: f64) -> f64 {
    let t121595 = 7.0_f64 / 288.0_f64 * t118588;
    let t121599 = 7.0_f64 / 1152.0_f64 * t118596;
    let t121601 = 7.0_f64 / 1152.0_f64 * t118602;
    let t121606 = 0.26915170729426927235e-3_f64 * t118586 + t121595 - t118590 / 192.0_f64 - t118592 / 192.0_f64 - t118594 / 192.0_f64 + t121599 + 0.16149102437656156341e-2_f64 * t112818 + t112821 + t112830 - t121601 + t114732 - t114734 - 0.96894614625936938046e-2_f64 * t118606 - t118608 / 768.0_f64 + t118610 / 192.0_f64 + t118612 / 192.0_f64 - t112847 + t114737 + t114739;
    t121606
}
