//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1205/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1205(t2163: f64, t5920: f64, t1518: f64, t29427: f64, t30137: f64, t30140: f64, t30142: f64, t30145: f64, t30147: f64, t30149: f64, t30716: f64, t30724: f64, t7586: f64) -> (f64, f64) {
    let t30951 = t2163 * t5920;
    let t30959 = 4.0_f64 * t1518 * t29427 + 2.0_f64 * t5920 * t7586 + t30137 + t30140 + t30142 + t30145 + t30147 + t30149 + t30716 + 2.0_f64 * t30724;
    (t30951, t30959)
}
