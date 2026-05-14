//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1204/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1204<F: Float>(t21603: F, t5932: F, t1124: F, t300: F, t179: F, t2739: F, t299: F, t5672: F, t771: F, t7765: F, t17946: F, t21454: F, t2104: F, t5974: F, t7719: F, t7649: F) -> (F, F, F, F, F, F, F) {
    let t21669 = t5932 * t21603;
    let t21686 = t300 * t1124;
    let t21714 = t299 * t179 * t5672 * t2739;
    let t21718 = t771 * t7765;
    let t21729 = t17946 * t21454;
    let t21746 = t2104 * t5974 * t7719;
    let t21749 = t2104 * t5974 * t7649;
    (t21669, t21686, t21714, t21718, t21729, t21746, t21749)
}
