//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2396/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2396(t246: f64, t40167: f64, t12126: f64, t588: f64, t39037: f64, t522: f64, t2221: f64, t3826: f64, t3824: f64, t12132: f64, t592: f64, t3696: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40168 = t40167 * t246;
    let t40221 = t588 * t12126;
    let t40224 = 840.0_f64 * t39037 * t522;
    let t40225 = t2221 * t3826;
    let t40227 = t2221 * t3824;
    let t40230 = 16.0_f64 * t592 * t12132;
    let t40231 = t2221 * t3696;
    (t40168, t40221, t40224, t40225, t40227, t40230, t40231)
}
