//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1239/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1239<F: Float>(t22575: F, t6006: F, t6007: F, t955: F, t2055: F, t2056: F, t2461: F, t2049: F, t2820: F, t2482: F, t6027: F, t6029: F, t22608: F, t7877: F, t22648: F, t22650: F, t897: F) -> (F, F, F, F, F, F, F) {
    let t26862 = 8.0 * t22575;
    let t26873 = t6006 * t955 * t6007;
    let t26881 = t2055 * t2461 * t2056;
    let t26882 = 0.1714584e0 * t26881;
    let t26886 = t2055 * t2820 * t2049;
    let t26917 = t6027 * t2482 * t6029;
    let t26918 = 0.4051561992e0 * t26917;
    let t26921 = t7877 * t22608;
    let t26922 = 0.4051561992e0 * t26921;
    let t26924 = t22648 * t897 * t22650;
    (t26862, t26873, t26882, t26886, t26918, t26922, t26924)
}
