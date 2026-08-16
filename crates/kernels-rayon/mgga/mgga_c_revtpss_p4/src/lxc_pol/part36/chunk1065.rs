//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1065/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1065(t24285: f64, t24322: f64, t1150: f64, t1131: f64, t12230: f64, t24220: f64, t12227: f64, t1744: f64, t6486: f64, t3479: f64, t16706: f64, t16876: f64, t20276: f64, t20278: f64, t20280: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24265: f64, t24267: f64, t24272: f64, t24275: f64) -> (f64, f64, f64, f64, f64) {
    let t24323 = t24285 + t24322;
    let t24324 = t24323 * t1150;
    let t24326 = 1.0_f64 * t1131 * t24324;
    let t24327 = t24220 * t12230;
    let t24329 = 0.51726012919273400301e3_f64 * t12227 * t24327;
    let t24330 = t6486 * t1744;
    let t24331 = t24330 * t3479;
    let t24348 = -0.52945875e1_f64 * t24265 + 0.94674375e0_f64 * t24267 + 0.68863333333333333332e0_f64 * t16706 + 0.34731666666666666667e0_f64 * t16876 + 0.46308888888888888889e-1_f64 * t24272 + 0.62517e0_f64 * t24275 + 0.69463333333333333335e-1_f64 * t20276 - 0.41678000000000000001e0_f64 * t20278 - 0.20839e0_f64 * t20280 + 0.34431666666666666666e0_f64 * t20283 - 0.103295e1_f64 * t20285 - 0.51647499999999999999e0_f64 * t20287 + 0.57386111111111111112e0_f64 * t24230 - 0.20659e1_f64 * t24234;
    (t24326, t24329, t24330, t24331, t24348)
}
