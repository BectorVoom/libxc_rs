//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 211/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk211(t143: f64, t130: f64, t131: f64, t72: f64, t122: f64, t125: f64) -> (f64, f64, f64, f64, f64) {
    let t680 = t143 * t143;
    let t681 = 1.0_f64 / t680;
    let t682 = t130 * t681;
    let t684 = 1.0_f64 / t131 * t72;
    let t685 = t122 * t125;
    (t680, t681, t682, t684, t685)
}
