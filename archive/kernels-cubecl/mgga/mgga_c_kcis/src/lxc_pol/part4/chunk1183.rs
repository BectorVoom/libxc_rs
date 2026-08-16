//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1183/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1183<F: Float>(t14920: F, t14960: F, t14998: F, t15053: F, t355: F, t377: F, t1817: F, t3369: F, t1809: F, t3358: F, t3436: F, t9531: F, sigma0: F) -> (F, F, F, F) {
    let t15055 = t14920 + t14960 + t14998 + t15053;
    let t15056 = t15055 * t355;
    let t15057 = t15056 * sigma0;
    let t15058 = t15057 * t377;
    let t15061 = t3369 * t1817;
    let t15063 = t1809 * t3358;
    let t15065 = t9531 * t3436;
    (t15058, t15061, t15063, t15065)
}
