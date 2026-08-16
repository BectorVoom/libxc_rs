//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1013/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1013<F: Float>(t14050: F, t4921: F, t3382: F, t4695: F, t1165: F, t3361: F, t3759: F, t4267: F, t1170: F, t15392: F) -> (F, F, F, F) {
    let t17118 = t14050 * t4921;
    let t17120 = t3382 * t4695;
    let t17128 = t3361 * t1165 * t4267 * t3759;
    let t17139 = t1170 * t15392;
    (t17118, t17120, t17128, t17139)
}
