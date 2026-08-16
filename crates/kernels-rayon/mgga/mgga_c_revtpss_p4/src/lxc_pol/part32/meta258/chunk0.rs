//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1083/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1083(t1372: f64, t7252: f64, t546: f64, t550: f64, t7028: f64, t807: f64, t2018: f64, t786: f64, t1381: f64, t1385: f64, t64: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7253 = t7252 * t1372;
    let t7256 = t546 * t7028 * t550;
    let t7257 = t807 * t7256;
    let t7259 = t786 * t2018;
    let t7260 = t7259 * t1381;
    let t7262 = t1385 * t64;
    (t7253, t7256, t7257, t7259, t7260, t7262)
}
