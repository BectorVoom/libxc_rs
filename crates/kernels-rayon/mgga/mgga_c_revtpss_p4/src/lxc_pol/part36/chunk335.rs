//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 335/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk335(t1524: f64, t1533: f64, t1536: f64, t225: f64, t679: f64, t704: f64, t751: f64, t759: f64, t764: f64, t1544: f64, t832: f64, t227: f64, t229: f64) -> (f64, f64, f64) {
    let t1553 = (t679 + t704 + t1524 + t1533 + t751 + t1536 - t759 - t764) * t225;
    let t1555 = t832 * t1544;
    let t1558 = -t1553 * t229 + 3.0_f64 * t1555 * t227;
    (t1553, t1555, t1558)
}
