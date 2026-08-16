//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1774/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1774(t12282: f64, t3417: f64, t141: f64, t3367: f64, t606: f64, t2258: f64) -> (f64, f64, f64) {
    let t12283 = t3417 * t12282;
    let t12284 = t141 * t12283;
    let t12286 = t3367 * t606;
    let t12287 = t12286 * t2258;
    (t12283, t12284, t12287)
}
