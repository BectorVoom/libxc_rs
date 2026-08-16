//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3211/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3211(t13321: f64, t13331: f64, t1480: f64, t21745: f64, t21754: f64, t2258: f64, t2270: f64, t2275: f64, t2282: f64, t2283: f64, t2286: f64, t44: f64, t46090: f64, t48: f64, t56: f64, t5835: f64, t5838: f64, t5843: f64, t60: f64, t60308: f64, t60311: f64, t60717: f64, t60754: f64, t60927: f64, t614: f64) -> f64 {
    let t60937 = -40.0_f64 / 9.0_f64 * t614 * t21745 + 5.0_f64 / 6.0_f64 * t44 * t48 * t60754 - 220.0_f64 / 27.0_f64 * t5843 * t2286 + 220.0_f64 / 81.0_f64 * t5843 * t2283 + 40.0_f64 / 9.0_f64 * t1480 * t13331 - 5.0_f64 / 6.0_f64 * t56 * t60 * t60754 + 5.0_f64 / 9.0_f64 * t56 * t2282 * t60717 + 220.0_f64 / 81.0_f64 * t2270 * t5835 + 5.0_f64 / 9.0_f64 * t44 * t2275 * t60717 + 220.0_f64 / 27.0_f64 * t2270 * t5838 - t46090 - 5.0_f64 / 27.0_f64 * t60308 * t60927 + 5.0_f64 / 27.0_f64 * t60311 * t60927 - 20.0_f64 / 81.0_f64 * t1480 * t13321 + 5.0_f64 / 108.0_f64 * t56 * t21754 * t2258;
    t60937
}
