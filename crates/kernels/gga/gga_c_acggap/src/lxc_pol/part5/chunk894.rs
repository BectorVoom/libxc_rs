//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 894/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk894<F: Float>(t1163: F, t1165: F, t16020: F, t540: F, t3382: F, t4991: F, t4983: F, t14047: F, t4277: F, t1111: F, t15947: F, t3361: F, t4355: F, t997: F, t13463: F, t171: F) -> (F, F, F, F, F, F, F) {
    let t16023 = t1163 * t1165 * t540 * t16020;
    let t16025 = t3382 * t4991;
    let t16044 = t3382 * t4983;
    let t16051 = t14047 * t4277;
    let t16055 = t3361 * t1165 * t15947 * t1111;
    let t16057 = t997 * t4355;
    let t16059 = t13463 * t171;
    (t16023, t16025, t16044, t16051, t16055, t16057, t16059)
}
