//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 816/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk816<F: Float>(t12266: F, t5677: F, t1464: F, t3734: F, t5881: F, t3801: F, t5632: F, t1395: F, t1394: F, t2001: F, t4136: F, t4135: F) -> (F, F, F, F, F) {
    let t15850 = t12266 * t5677;
    let t15851 = t1464 * t15850;
    let t15853 = t3734 * t5881;
    let t15854 = t1464 * t15853;
    let t15856 = t5632 * t3801;
    let t15857 = t1395 * t15856;
    let t15858 = t1394 * t15857;
    let t15860 = t2001 * t4136;
    let t15861 = t4135 * t15860;
    (t15851, t15854, t15858, t15860, t15861)
}
