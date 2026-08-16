//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 643/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk643(t225: f64, t2735: f64, t826: f64, t849: f64, t820: f64, t823: f64, t843: f64) -> (f64, f64, f64, f64) {
    let t2736 = t2735 * t225;
    let t2737 = t849 * t826;
    let t2739 = 0.25410001404642664112e-5_f64 * t2736 * t2737;
    let t2741 = t820 * t823 * t843;
    (t2736, t2737, t2739, t2741)
}
