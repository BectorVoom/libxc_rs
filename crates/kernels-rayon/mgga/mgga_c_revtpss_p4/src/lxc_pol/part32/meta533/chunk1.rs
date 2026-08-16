//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1840/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1840(t2018: f64, t9646: f64, t9723: f64, t26014: f64, t2689: f64, t3994: f64, t7028: f64, t9845: f64, t25240: f64, t3951: f64, t3964: f64, t2681: f64, t7269: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t94525 = t9646 * t2018 * t9723;
    let t94527 = t2689 * t26014;
    let t94537 = t9845 * t7028 * t3994;
    let t94540 = t3964 * t25240 * t3951;
    let t94545 = t820 * t7269 * t2681;
    (t94525, t94527, t94537, t94540, t94545)
}
