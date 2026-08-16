//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1280/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1280(t120672: f64, t120675: f64, t120677: f64, t120678: f64, t120680: f64, t120683: f64, t120687: f64, t120691: f64, t120692: f64, t120697: f64, t120699: f64, t120702: f64, t120703: f64, t120708: f64, t120709: f64, t120711: f64, t24999: f64, t25965: f64, t6517: f64, t6539: f64) -> f64 {
    let t120713 = -4.0_f64 * t24999 * t6539 - 4.0_f64 * t25965 * t6517 - t120672 + 2.0_f64 * t120675 - t120677 - 4.0_f64 * t120678 - 4.0_f64 * t120680 - t120683 - t120687 - t120691 + 6.0_f64 * t120692 + t120697 + t120699 + t120702 + 6.0_f64 * t120703 - t120708 - 4.0_f64 * t120709 - 4.0_f64 * t120711;
    t120713
}
