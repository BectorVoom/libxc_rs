//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1206/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1206<F: Float>(t17867: F, t2104: F, t2932: F, t7607: F, t7784: F, t2945: F, t2947: F, t5939: F, t2099: F, t7788: F, t7792: F, t7797: F, t757: F, t7577: F, t2908: F, t5945: F) -> (F, F, F, F, F, F, F, F) {
    let t21862 = t2104 * t17867 * t2932;
    let t21867 = t7607 * t7784;
    let t21870 = t2945 * t5939 * t2947;
    let t21874 = t2945 * t2099 * t7788;
    let t21877 = t2945 * t2099 * t7792;
    let t21882 = t2945 * t2099 * t7797;
    let t21928 = t757 * t2099 * t7577;
    let t21930 = t5945 * t2908;
    (t21862, t21867, t21870, t21874, t21877, t21882, t21928, t21930)
}
