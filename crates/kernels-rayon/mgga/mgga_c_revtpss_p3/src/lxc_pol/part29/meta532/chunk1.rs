//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1863/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1863(t25310: f64, t26506: f64, t26485: f64, t93364: f64, t2829: f64, t689: f64, t7384: f64, t2439: f64, t7398: f64, t780: f64, t785: f64, t93134: f64, t95546: f64) -> (f64, f64, f64, f64, f64) {
    let t95551 = t25310 * t26506;
    let t95553 = t93364 * t26485;
    let t95556 = t689 * t7384 * t2829;
    let t95562 = t2439 * t785 * t7398 * t780;
    let t95567 = 0.43639970290213137151e-3_f64 * t93134 * t95546;
    (t95551, t95553, t95556, t95562, t95567)
}
