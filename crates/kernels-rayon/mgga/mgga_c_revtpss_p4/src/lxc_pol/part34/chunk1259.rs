//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1259/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1259(t30056: f64, t686: f64, t72: f64, t7289: f64, t7284: f64, t30100: f64, t689: f64, t25904: f64, t25899: f64, t30031: f64, t25878: f64, t108278: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108307 = t30056 * t72 * t686;
    let t108308 = t7289 * t108307;
    let t108332 = t7284 * t108307;
    let t108334 = t30100 * t689;
    let t108335 = t25904 * t108334;
    let t108337 = t25899 * t108334;
    let t108368 = t30031 * t72 * t686;
    let t108369 = t25878 * t108368;
    let t108379 = t786 * t108278;
    (t108308, t108332, t108335, t108337, t108368, t108369, t108379)
}
