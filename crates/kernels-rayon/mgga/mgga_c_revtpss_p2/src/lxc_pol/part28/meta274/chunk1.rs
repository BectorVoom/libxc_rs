//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1228/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1228(t7076: f64, t7774: f64, t233: f64, t7759: f64, t1957: f64, t1580: f64, t1956: f64, t1959: f64, t213: f64, t257: f64, t7017: f64, t7020: f64, t7053: f64, t7062: f64, t7066: f64, t7070: f64, t7760: f64, t7766: f64, t7770: f64) -> (f64, f64, f64, f64) {
    let t7775 = t7076 * t7774;
    let t7778 = t233 * t7759;
    let t7779 = t1957 * t7778;
    let t7782 = -t7017 + t7020 + 0.65854491829355115987e0_f64 * t213 * t7760 * t257 - 0.65854491829355115987e0_f64 * t7053 * t1580 + t7062 - t7066 - 0.4336814094102599731e0_f64 * t7766 * t1959 + 0.8673628188205199462e0_f64 * t7070 * t7770 + 0.4336814094102599731e0_f64 * t7070 * t7775 - 0.4336814094102599731e0_f64 * t1956 * t7779;
    (t7775, t7778, t7779, t7782)
}
