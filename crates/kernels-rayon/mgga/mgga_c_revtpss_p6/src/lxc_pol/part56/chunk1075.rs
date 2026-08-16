//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1075/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1075(t33505: f64, t33521: f64, t1245: f64, t33515: f64, t33516: f64, t72: f64, t120199: f64, t33425: f64, t33427: f64, t33404: f64, t33424: f64, t2148: f64, t33454: f64, t3736: f64) -> (f64, f64, f64, f64, f64) {
    let t124719 = t33505 * t33521;
    let t124744 = t33515 * t33516 * t1245 * t72;
    let t124748 = t33425 * t120199 * t33427;
    let t124755 = t33404 * t33424;
    let t124772 = t2148 * t33454 * t3736;
    (t124719, t124744, t124748, t124755, t124772)
}
