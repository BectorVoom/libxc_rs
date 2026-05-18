//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1076/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1076<F: Float>(t3055: F, t4245: F, t1160: F, t1629: F, t17581: F, t377: F, t5322: F, t1647: F, t3036: F, t3037: F, t15407: F, t3073: F) -> (F, F, F, F, F) {
    let t19176 = t3055 * t4245;
    let t19179 = t1160 * t1629 * t17581;
    let t19181 = t377 * t5322;
    let t19196 = t3036 * t1647 * t3037;
    let t19199 = t3073 * t1629 * t15407;
    (t19176, t19179, t19181, t19196, t19199)
}
