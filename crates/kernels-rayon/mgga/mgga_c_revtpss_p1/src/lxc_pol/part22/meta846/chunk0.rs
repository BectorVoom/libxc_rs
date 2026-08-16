//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2983/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2983(t1882: f64, t2482: f64, t4104: f64, t4118: f64, t1398: f64, t2782: f64, t4086: f64, t543: f64, t5710: f64, t1897: f64, t40317: f64, t10111: f64, t22: f64, t5759: f64) -> (f64, f64, f64, f64) {
    let t49325 = t2482 * t4118 * t1882 * t4104;
    let t49346 = t2782 * t4086 * t5710 * t1398 * t543;
    let t49354 = t40317 * t1897;
    let t49361 = t10111 * t5759 * t22;
    (t49325, t49346, t49354, t49361)
}
