//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 971/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk971<F: Float>(t1165: F, t14373: F, t15758: F, t530: F, t14368: F, t1532: F, t322: F, t4919: F, t3382: F, t5209: F, t5213: F, t3431: F, t5281: F) -> (F, F, F, F, F) {
    let t15761 = t14373 * t1165 * t530 * t15758;
    let t15774 = t14368 * t1165 * t1532 * t4919 * t322;
    let t15776 = t3382 * t5209;
    let t15787 = t3382 * t5213;
    let t15789 = t3431 * t5281;
    (t15761, t15774, t15776, t15787, t15789)
}
