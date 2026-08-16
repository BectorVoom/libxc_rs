//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1676/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1676(t2408: f64, t30: f64, t605: f64, t890: f64, t2832: f64, t2394: f64, t33: f64, t2411: f64) -> (f64, f64, f64, f64, f64) {
    let t25446 = t30 * t2408;
    let t25449 = t605 * t890;
    let t25452 = t30 * t2832;
    let t25752 = t33 * t2394;
    let t25759 = t2411 * t33;
    (t25446, t25449, t25452, t25752, t25759)
}
