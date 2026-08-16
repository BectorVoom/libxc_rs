//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3298/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3298(t2782: f64, t4086: f64, t543: f64, t86455: f64, t86470: f64, t14192: f64, t86445: f64, t9994: f64, t22964: f64, t545: f64, t689: f64, t869: f64) -> (f64, f64, f64, f64) {
    let t86575 = t2782 * t4086 * t86455 * t543;
    let t86582 = t2782 * t4086 * t86470 * t543;
    let t86586 = t2782 * t14192 * t86445 * t9994;
    let t86597 = t689 * t869 * t545 * t22964;
    (t86575, t86582, t86586, t86597)
}
