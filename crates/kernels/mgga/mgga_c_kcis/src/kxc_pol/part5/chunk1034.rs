//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1034/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1034<F: Float>(t15516: F, t3514: F, t421: F, t4951: F, t3490: F, t5299: F, t11061: F, t1846: F, t1251: F, t2470: F, t992: F, t5315: F) -> (F, F, F, F, F) {
    let t15518 = t3514 * t15516 / F::cast_from(864.0_f64);
    let t15534 = t4951 * t421;
    let t15547 = t3490 * t5299 / F::cast_from(324.0_f64);
    let t15548 = t11061 * t1846;
    let t15549 = t1251 * t15548;
    let t15553 = t2470 * t992;
    let t15554 = t15553 * t5315;
    (t15518, t15534, t15547, t15549, t15554)
}
