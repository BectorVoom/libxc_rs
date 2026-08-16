//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1220/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1220(t5: f64, t1923: f64, t1928: f64, t6958: f64, t7702: f64, t7706: f64, t7709: f64, t7716: f64, t7720: f64, t117: f64, t1937: f64, t4248: f64, t1518: f64, t94: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7724 = piecewise3(t8, 0.0_f64, -t7702 * t1928 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t6958 * t7706 + t7709 * t1928 / 3.0_f64 - t1923 * t7716 / 6.0_f64 - t1923 * t7720 / 6.0_f64);
    let t7725 = t7724 * t117;
    let t7731 = 2.0_f64 * t4248 * t1937;
    let t7732 = t94 * t1518;
    (t7724, t7725, t7731, t7732)
}
