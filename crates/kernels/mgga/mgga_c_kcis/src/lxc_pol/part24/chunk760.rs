//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 760/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk760<F: Float>(t25: F, t5337: F, t1251: F, t11081: F, t5325: F, t3514: F, t421: F, t4951: F, t3490: F, t5299: F, t11061: F, t1846: F, t2470: F, t992: F, t5315: F, t287: F) -> (F, F, F, F, F, F, F, F) {
    let t15494 = t25 * t5337;
    let t15496 = t1251 * t15494 / 288.0;
    let t15516 = t11081 * t5325;
    let t15518 = t3514 * t15516 / 864.0;
    let t15534 = t4951 * t421;
    let t15547 = t3490 * t5299 / 324.0;
    let t15548 = t11061 * t1846;
    let t15549 = t1251 * t15548;
    let t15553 = t2470 * t992;
    let t15554 = t15553 * t5315;
    let t15555 = t1251 * t15554;
    let t15573 = t25 * t287;
    (t15496, t15518, t15534, t15547, t15549, t15553, t15555, t15573)
}
