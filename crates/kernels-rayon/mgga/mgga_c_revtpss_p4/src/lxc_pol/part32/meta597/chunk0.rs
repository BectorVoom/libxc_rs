//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1930/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1930(t2435: f64, t28448: f64, t28314: f64, t93364: f64, t103431: f64, t25375: f64, t212: f64, t28340: f64, t689: f64, t780: f64, t103182: f64, t93281: f64) -> (f64, f64, f64, f64, f64) {
    let t103490 = t2435 * t28448;
    let t103494 = 0.28912093960683998208e-1_f64 * t93364 * t28314;
    let t103521 = t25375 * t103431;
    let t103529 = 0.10975748638225852664e-1_f64 * t689 * t212 * t28340 * t780;
    let t103543 = t93281 * t103182;
    (t103490, t103494, t103521, t103529, t103543)
}
