//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1029/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1029<F: Float>(t113: F, t8538: F, t9064: F, t2605: F, t7627: F, t2491: F, t2593: F, t740: F, t2588: F, t26533: F, t2526: F, t808: F, t9053: F, t2150: F, t755: F, t8750: F) -> (F, F, F, F, F, F, F) {
    let t91799 = t9064 * t113 * t8538;
    let t91801 = t2605 * t7627;
    let t91804 = t2593 * t740 * t2491;
    let t91806 = t2588 * t26533;
    let t91809 = t808 * t740 * t2526;
    let t91811 = t2593 * t9053;
    let t91814 = t755 * t2150 * t8750;
    (t91799, t91801, t91804, t91806, t91809, t91811, t91814)
}
