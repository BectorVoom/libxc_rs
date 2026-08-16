//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1389/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1389(t14473: f64, t2439: f64, t212: f64, t4469: f64, t780: f64, t689: f64, t1579: f64, t2769: f64, t886: f64, t252: f64, t2782: f64, t2470: f64, t4480: f64) -> (f64, f64, f64, f64, f64) {
    let t14474 = t2439 * t14473;
    let t14476 = t212 * t4469;
    let t14477 = t14476 * t780;
    let t14479 = 0.10975748638225852664e-1_f64 * t689 * t14477;
    let t14480 = t2769 * t1579;
    let t14481 = t14480 * t886;
    let t14482 = t252 * t14481;
    let t14484 = 0.21951497276451705328e-1_f64 * t2782 * t14482;
    let t14485 = t4480 * t2470;
    (t14474, t14479, t14481, t14484, t14485)
}
