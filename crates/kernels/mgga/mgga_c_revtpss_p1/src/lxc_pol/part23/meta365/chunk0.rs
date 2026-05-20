//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1682/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1682<F: Float>(t15191: F, t15197: F, t4682: F, t964: F, t1626: F, t3011: F, t15125: F, t11387: F, t1609: F, t4644: F, t945: F, t1614: F, t2967: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15322 = F::cast_from(0.34431666666666666666e0_f64) * t15191;
    let t15324 = F::cast_from(0.13892666666666666667e0_f64) * t15197;
    let t15343 = t4682 * t964;
    let t15350 = t1626 * t3011;
    let t15363 = F::cast_from(0.2283111111111111111e-1_f64) * t15125;
    let t15364 = F::cast_from(0.11415555555555555555e-1_f64) * t15191;
    let t15396 = t1609 * t11387;
    let t15400 = t4644 * t945;
    let t15406 = t1614 * t2967;
    (t15322, t15324, t15343, t15350, t15363, t15364, t15396, t15400, t15406)
}
