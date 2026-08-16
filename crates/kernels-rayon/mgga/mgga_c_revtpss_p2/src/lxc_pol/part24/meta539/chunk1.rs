//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1586/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586(t22964: f64, t545: f64, t689: f64, t869: f64, t2782: f64, t4086: f64, t543: f64, t86506: f64, t86445: f64, t4003: f64, t5744: f64, t86470: f64) -> (f64, f64, f64, f64) {
    let t86597 = t689 * t869 * t545 * t22964;
    let t86604 = t2782 * t4086 * t86506 * t543;
    let t86608 = t2782 * t4086 * t86445 * t543;
    let t86634 = t2782 * t5744 * t86470 * t4003;
    (t86597, t86604, t86608, t86634)
}
