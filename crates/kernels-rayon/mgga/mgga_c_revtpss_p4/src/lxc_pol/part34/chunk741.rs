//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 741/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk741(t651: f64, t7742: f64, t1518: f64, t2007: f64, t1544: f64, t30: f64, t1963: f64, t1549: f64, t7025: f64, t1561: f64, t7038: f64, t1565: f64, t7045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7744 = 2.0_f64 * t651 * t7742;
    let t7746 = t2007 * t1518;
    let t7749 = t30 * t1544;
    let t7750 = t1963 * t7749;
    let t7753 = t7025 * t1549;
    let t7755 = t7038 * t1561;
    let t7757 = t7045 * t1565;
    (t7744, t7746, t7749, t7750, t7753, t7755, t7757)
}
