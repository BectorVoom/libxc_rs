//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 518/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk518<F: Float>(t1504: F, t4189: F, t3502: F, t381: F, t498: F, t493: F, t3732: F, t470: F, t487: F, t1487: F, t1488: F, t1492: F) -> (F, F, F, F, F, F, F, F) {
    let t4190 = t1504 * t4189;
    let t4192 = t381 * t3502;
    let t4193 = t498 * t4192;
    let t4194 = t493 * t4193;
    let t4196 = t470 * t3732;
    let t4197 = t487 * t4196;
    let t4198 = t1487 * t4197;
    let t4200 = t1492 * t1488;
    (t4190, t4192, t4193, t4194, t4196, t4197, t4198, t4200)
}
