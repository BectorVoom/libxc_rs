//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1923/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1923(t14720: f64, t11215: f64, t11217: f64, t14722: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t14766: f64, t11219: f64, t14726: f64) -> (f64, f64, f64) {
    let t14768 = 0.13418888888888888889e0_f64 * t14720;
    let t14776 = -0.11038e0_f64 * t11215 - 0.5519e-1_f64 * t11217 + 0.91983333333333333334e-1_f64 * t14766 + t14768 - 0.40256666666666666666e0_f64 * t14738 - 0.20128333333333333333e0_f64 * t14742 - 0.12077e1_f64 * t14733 + 0.12077e1_f64 * t14751 + 0.60385e0_f64 * t14755 + 0.181155e1_f64 * t14746 - 0.40256666666666666667e0_f64 * t14722;
    let t14778 = t11219 * t14726;
    (t14768, t14776, t14778)
}
