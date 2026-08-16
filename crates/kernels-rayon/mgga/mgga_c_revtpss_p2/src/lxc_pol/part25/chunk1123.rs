//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1123/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1123(t1949: f64, t25317: f64, t2771: f64, t213: f64, t7048: f64, t2828: f64, t7071: f64, t2470: f64, t7059: f64, t7064: f64, t785: f64, t780: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25319 = t25317 * t1949 * t2771;
    let t25322 = t213 * t7048;
    let t25325 = t1949 * t2828;
    let t25326 = t7071 * t25325;
    let t25331 = t7059 * t2470;
    let t25333 = 0.17135234354032049604e-1_f64 * t7064 * t25331;
    let t25334 = t785 * t1949;
    let t25335 = t25334 * t780;
    (t25319, t25322, t25325, t25326, t25331, t25333, t25334, t25335)
}
