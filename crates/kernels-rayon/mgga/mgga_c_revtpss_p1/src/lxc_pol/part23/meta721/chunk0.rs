//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2482/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2482(t2482: f64, t814: f64, t9991: f64, t46917: f64, t5706: f64, t241: f64, t47201: f64, t820: f64, t47198: f64, t5665: f64, t5629: f64, t9779: f64) -> (f64, f64, f64, f64, f64) {
    let t48731 = t2482 * t9991 * t814;
    let t48756 = t46917 * t5706;
    let t48759 = t820 * t47201 * t241;
    let t48792 = t47198 * t5665;
    let t48794 = t9779 * t5629;
    (t48731, t48756, t48759, t48792, t48794)
}
