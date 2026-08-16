//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1225/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1225<F: Float>(t6019: F, t1498: F, t1464: F, t11783: F, t2002: F, t3954: F, t5632: F, t1468: F, t4124: F, t4123: F, t3734: F, t5633: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t15808 = t6019 * sigma2;
    let t15809 = t15808 * t1498;
    let t15810 = t1464 * t15809;
    let t15812 = t11783 * t2002;
    let t15813 = t1464 * t15812;
    let t15815 = t5632 * t3954;
    let t15816 = t1468 * t15815;
    let t15817 = t1464 * t15816;
    let t15819 = t5632 * t4124;
    let t15820 = t4123 * t15819;
    let t15821 = t1464 * t15820;
    let t15823 = t3734 * t5633;
    (t15808, t15810, t15813, t15817, t15821, t15823)
}
