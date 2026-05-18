//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 877/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk877<F: Float>(t3372: F, t3665: F, t1162: F, t12313: F, t1037: F, t1165: F, t945: F, t1160: F, t3430: F, t3198: F, t1111: F, t301: F) -> (F, F, F, F, F, F) {
    let t12770 = t3372 * t3665;
    let t12801 = t12313 * t1162;
    let t12804 = t12801 * t1165 * t1037 * t945;
    let t12813 = t1160 * t3430;
    let t12814 = t12813 * t3198;
    let t12816 = t1111 * t301;
    (t12770, t12801, t12804, t12813, t12814, t12816)
}
