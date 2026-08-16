//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1847/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1847<F: Float>(t1214: F, t3603: F, t12810: F, t3720: F, t1250: F, t12726: F, t11772: F, t3623: F) -> (F, F, F, F, F) {
    let t12856 = t3603 * t1214;
    let t12857 = t12810 * t12856;
    let t12858 = t3720 * t12857;
    let t12861 = t12726 * t1250;
    let t12862 = t3720 * t12861;
    let t12865 = t3623 * t11772;
    (t12857, t12858, t12861, t12862, t12865)
}
