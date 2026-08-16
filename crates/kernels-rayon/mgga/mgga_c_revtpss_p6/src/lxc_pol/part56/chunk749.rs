//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 749/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk749(t532: f64, t8598: f64, t4147: f64, t2014: f64, t116: f64, t8453: f64, t572: f64, t117: f64, t8460: f64, t136: f64, t8440: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8599 = t532 * t8598;
    let t8600 = t8599 * t4147;
    let t8601 = t2014 * t8600;
    let t8611 = t116 * t8453;
    let t8613 = 6.0_f64 * t572 * t8611;
    let t8614 = t117 * t8460;
    let t8616 = 3.0_f64 * t572 * t8614;
    let t8621 = t136 * t8440;
    (t8599, t8600, t8601, t8611, t8613, t8614, t8616, t8621)
}
