//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1586/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586<F: Float>(t22964: F, t545: F, t689: F, t869: F, t2782: F, t4086: F, t543: F, t86506: F, t86445: F, t4003: F, t5744: F, t86470: F) -> (F, F, F, F) {
    let t86597 = t689 * t869 * t545 * t22964;
    let t86604 = t2782 * t4086 * t86506 * t543;
    let t86608 = t2782 * t4086 * t86445 * t543;
    let t86634 = t2782 * t5744 * t86470 * t4003;
    (t86597, t86604, t86608, t86634)
}
