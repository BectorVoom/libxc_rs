//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1091/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1091(t124: f64, t867: f64, t14686: f64, t886: f64, t119836: f64, t1032: f64, t2735: f64, t119867: f64, t233: f64, t240: f64, t31838: f64, t31840: f64, t845: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119891 = t124 * t867;
    let t119893 = t14686 * t119891 * t886;
    let t119894 = t119836 * t119893;
    let t119900 = t2735 * t1032;
    let t119903 = t119900 * t233 * t240 * t119867;
    let t119904 = 0.18822977838986977999e-5_f64 * t119903;
    let t119912 = t31838 * t845 * t31840;
    (t119891, t119893, t119894, t119900, t119904, t119912)
}
