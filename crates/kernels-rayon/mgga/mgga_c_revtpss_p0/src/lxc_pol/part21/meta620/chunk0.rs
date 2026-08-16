//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2376/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376(t10815: f64, t2648: f64, t2756: f64, t2681: f64, t2719: f64, t820: f64, t2726: f64, t10850: f64, t10861: f64, t221: f64, t2485: f64, t10111: f64, t823: f64, t9720: f64) -> (f64, f64, f64, f64, f64) {
    let t40393 = t10815 * t2648;
    let t40395 = t10815 * t2756;
    let t40398 = t820 * t2719 * t2681;
    let t40399 = t40398 * t2726;
    let t40403 = t10850 * t2485 * t221 * t10861;
    let t40406 = t10111 * t823 * t9720;
    (t40393, t40395, t40399, t40403, t40406)
}
