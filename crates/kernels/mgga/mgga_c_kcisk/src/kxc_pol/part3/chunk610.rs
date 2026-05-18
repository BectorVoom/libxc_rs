//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 610/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk610<F: Float>(t5052: F, t5210: F, t752: F, t1904: F, t1907: F, t1957: F, t1906: F, t751: F, t724: F, t196: F, t4794: F, t574: F, t725: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5211 = t5052 + t5210;
    let t5212 = t5211 * t752;
    let t5213 = t1904 * t1907;
    let t5215 = F::new(2.0) * t5213 * t1957;
    let t5217 = F::new(1.0) / t1906 / t751;
    let t5218 = t724 * t5217;
    let t5219 = t1957 * t1957;
    let t5221 = F::new(2.0) * t5218 * t5219;
    let t5222 = t4794 * t196;
    let t5231 = t725 * t574;
    (t5211, t5212, t5213, t5215, t5217, t5218, t5219, t5221, t5222, t5231)
}
