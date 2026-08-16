//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 180/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk180(t755: f64, t757: f64, t177: f64, t192: f64, t738: f64, t744: f64, t745: f64) -> (f64, f64, f64) {
    let t759 = 0.18311447306006545054e-3_f64 * t755 * t757;
    let t760 = t192 * t177;
    let t762 = t738 * t744 * t745;
    (t759, t760, t762)
}
