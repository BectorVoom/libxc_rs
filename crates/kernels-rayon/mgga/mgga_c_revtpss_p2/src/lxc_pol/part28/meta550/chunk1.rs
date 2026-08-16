//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2000/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000(t1926: f64, t92576: f64, t1927: f64, t2315: f64, t2247: f64, t2259: f64, t2269: f64, t48: f64, t2275: f64, t613: f64, t10355: f64, t43: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92577 = t1926 * t92576;
    let t92584 = t1927 * t2315;
    let t92585 = t1926 * t92584;
    let t92588 = t2247 * t2259;
    let t92597 = t2269 * t48;
    let t92600 = t613 * t2275;
    let t92605 = t43 * t10355;
    (t92577, t92585, t92588, t92597, t92600, t92605)
}
