//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1389/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1389(t1248: f64, t6587: f64, t1250: f64, t3720: f64, t17183: f64, t5330: f64, t17737: f64, t5297: f64, t3626: f64, t1230: f64, t6594: f64, t1803: f64, t5261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21298 = t6587 * t1248;
    let t21299 = t21298 * t1250;
    let t21300 = t3720 * t21299;
    let t21306 = t17183 * t5330;
    let t21309 = t17737 * t5297;
    let t21310 = t3626 * t21309;
    let t21313 = t1230 * t6594;
    let t21316 = t5261 * t1803;
    (t21298, t21300, t21306, t21310, t21313, t21316)
}
