//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3298/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3298<F: Float>(t2782: F, t4086: F, t543: F, t86455: F, t86470: F, t14192: F, t86445: F, t9994: F, t22964: F, t545: F, t689: F, t869: F) -> (F, F, F, F) {
    let t86575 = t2782 * t4086 * t86455 * t543;
    let t86582 = t2782 * t4086 * t86470 * t543;
    let t86586 = t2782 * t14192 * t86445 * t9994;
    let t86597 = t689 * t869 * t545 * t22964;
    (t86575, t86582, t86586, t86597)
}
