//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1573/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1573<F: Float>(t10785: F, t2747: F, t2749: F, t125: F, t2645: F, t4364: F, t4366: F, t837: F, t820: F, t823: F, t844: F) -> (F, F, F, F, F) {
    let t10794 = t2747 * t10785 * t2749;
    let t10797 = t125 * t2645;
    let t10799 = t4364 * t10797 * t4366;
    let t10803 = t2747 * t10797 * t2749;
    let t10807 = t4364 * t10797 * t837;
    let t10811 = t820 * t823 * t844;
    (t10794, t10799, t10803, t10807, t10811)
}
