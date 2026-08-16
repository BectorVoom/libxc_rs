//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1777/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1777(t1949: f64, t25317: f64, t2771: f64, t213: f64, t7048: f64, t2828: f64, t7071: f64, t2470: f64, t7059: f64) -> (f64, f64, f64, f64, f64) {
    let t25319 = t25317 * t1949 * t2771;
    let t25322 = t213 * t7048;
    let t25325 = t1949 * t2828;
    let t25326 = t7071 * t25325;
    let t25331 = t7059 * t2470;
    (t25319, t25322, t25325, t25326, t25331)
}
