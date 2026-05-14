//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 975/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk975<F: Float>(t3233: F, t7726: F, t303: F, t10463: F, t1250: F, t291: F, t9985: F, t2809: F, t5329: F, t1014: F, t7735: F, t1009: F, t2909: F, t2911: F, t1086: F, t1094: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26778 = t7726 * t3233;
    let t26779 = t303 * t26778;
    let t26781 = t10463 * t1250;
    let t26782 = t291 * t9985;
    let t26783 = t26782 * t2809;
    let t26784 = t5329 * t26783;
    let t26787 = t1014 * t7735;
    let t26791 = t2909 * t1009;
    let t26792 = t26791 * t2911;
    let t26793 = t5329 * t26792;
    let t26796 = t1086 * t1094;
    (t26778, t26779, t26781, t26783, t26784, t26787, t26792, t26793, t26796)
}
