//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 968/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk968<F: Float>(t1163: F, t1165: F, t4210: F, t4289: F, t1137: F, t4769: F, t12936: F, t3655: F, t4417: F, t3044: F, t540: F, t4254: F, t8887: F) -> (F, F, F, F, F) {
    let t15653 = t1163 * t1165 * t4289 * t4210;
    let t15667 = t1137 * t4769;
    let t15671 = t12936 * t1165 * t4417 * t3655;
    let t15675 = t12936 * t1165 * t540 * t3044;
    let t15690 = t4254 * t8887;
    (t15653, t15667, t15671, t15675, t15690)
}
