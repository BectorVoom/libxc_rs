//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 199/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk199(t625: f64, t44: f64, t49: f64, t56: f64, t614: f64, t617: f64, t620: f64, t38: f64, t45: f64) -> (f64, f64, f64, f64) {
    let t626 = 8.0_f64 / 3.0_f64 * t625;
    let t627 = -8.0_f64 / 3.0_f64 * t614 * t49 + 5.0_f64 / 6.0_f64 * t44 * t617 - 5.0_f64 / 6.0_f64 * t56 * t620 + t626;
    let t628 = t38 * t627;
    let t631 = t45 * t45;
    (t626, t627, t628, t631)
}
