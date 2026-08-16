//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 964/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk964<F: Float>(t22043: F, t22089: F, t22146: F, t22210: F, t1396: F, t1468: F, t1464: F, t1489: F, t21886: F, t1494: F, t7052: F, t1497: F) -> (F, F, F, F, F, F) {
    let t22212 = t22043 + t22089 + t22146 + t22210;
    let t22213 = t1396 * t22212;
    let t22214 = t1468 * t22213;
    let t22215 = t1464 * t22214;
    let t22219 = t21886 * t1489;
    let t22220 = t1468 * t22219;
    let t22221 = t1464 * t22220;
    let t22223 = t7052 * t1494;
    let t22224 = t22223 * t1497;
    (t22212, t22213, t22215, t22219, t22221, t22224)
}
