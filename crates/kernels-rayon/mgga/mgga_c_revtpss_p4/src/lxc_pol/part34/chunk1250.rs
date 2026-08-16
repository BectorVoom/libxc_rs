//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1250/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1250(t19462: f64, t1976: f64, t106655: f64, t7150: f64, t19658: f64, t7122: f64, t19920: f64, t25522: f64, t27489: f64, t4817: f64, t19882: f64, t7132: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106727 = t19462 * t1976;
    let t106787 = t7150 * t106655;
    let t106877 = t7122 * t19658;
    let t106896 = t25522 * t19920;
    let t106906 = t27489 * t4817;
    let t106923 = t7132 * t19882;
    (t106727, t106787, t106877, t106896, t106906, t106923)
}
