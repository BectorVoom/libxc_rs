//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1170/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1170(t7235: f64, t7313: f64, t2322: f64, t7003: f64, t18163: f64, t1937: f64, t4254: f64, t6993: f64, t7239: f64, t25832: f64, t508: f64, t651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25838 = 2.0_f64 * t7235 * t7313;
    let t25840 = 4.0_f64 * t2322 * t7003;
    let t25842 = 2.0_f64 * t18163 * t1937;
    let t25844 = 4.0_f64 * t4254 * t6993;
    let t25846 = 6.0_f64 * t7235 * t7239;
    let t25851 = t508 * t25832;
    let t25853 = 2.0_f64 * t651 * t25851;
    (t25838, t25840, t25842, t25844, t25846, t25851, t25853)
}
