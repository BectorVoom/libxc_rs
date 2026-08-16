//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1511/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1511<F: Float>(t12072: F, t1649: F, t2: F, t3672: F, t1787: F, t2516: F, t17: F, t12120: F, t2663: F, t5157: F, t1788: F, t2225: F) -> (F, F, F, F, F, F, F) {
    let t15952 = t12072 * t1649;
    let t15955 = t3672 * t2;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15976 = F::cast_from(4.0_f64) * t12120;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    (t15952, t15955, t15971, t15972, t15976, t15979, t15982)
}
