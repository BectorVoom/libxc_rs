//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1077/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1077(t26291: f64, t26374: f64, t532: f64, t1450: f64, t1310: f64, t18163: f64, t2014: f64, t2056: f64, t2089: f64, t2093: f64, t2320: f64, t2322: f64, t2328: f64, t2372: f64, t26154: f64, t26162: f64, t26210: f64, t26218: f64, t26223: f64, t4151: f64, t4254: f64, t508: f64, t649: f64, t651: f64, t7235: f64, t7357: f64, t7359: f64, t7367: f64, t7374: f64, t7378: f64, t7474: f64, t7489: f64, t7539: f64) -> (f64, f64, f64, f64) {
    let t26375 = t26291 + t26374;
    let t26376 = t532 * t26375;
    let t26377 = t26376 * t1450;
    let t26379 = -2.0_f64 * t1310 * t7357 - 2.0_f64 * t18163 * t2056 + 6.0_f64 * t2014 * t26162 + t2014 * t26377 - t2089 * t2320 - 2.0_f64 * t2089 * t2328 + t2093 * t4151 - 4.0_f64 * t2322 * t7374 - 4.0_f64 * t2322 * t7378 - 2.0_f64 * t2372 * t7359 - 2.0_f64 * t26154 * t651 - t26210 * t508 - 2.0_f64 * t26218 * t651 - 4.0_f64 * t26223 * t651 - 4.0_f64 * t4254 * t7367 - 4.0_f64 * t4254 * t7374 - 2.0_f64 * t649 * t7474 + 6.0_f64 * t7235 * t7489 - 2.0_f64 * t7235 * t7539;
    (t26375, t26376, t26377, t26379)
}
