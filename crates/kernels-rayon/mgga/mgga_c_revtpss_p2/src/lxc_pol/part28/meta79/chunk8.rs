//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 511/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk511(t1524: f64, t1533: f64, t1536: f64, t1544: f64, t1583: f64, t198: f64, t207: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64, t765: f64, t892: f64) -> f64 {
    let t1587 = t1583 * t198 * t207 * t892 + 3.0_f64 * t1544 * t198 * t765 + t1524 + t1533 + t1536 + t679 + t704 + t751 - t759 - t764;
    t1587
}
