//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1025/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1025<F: Float>(t1960: F, t3073: F, t3322: F, t8440: F, t27229: F, t9777: F, t10805: F, t7324: F, t8862: F, t9780: F, t1052: F, t29646: F) -> (F, F, F, F, F, F) {
    let t44231 = F::cast_from(2.0_f64) * t1960 * t3073 * t3322;
    let t44232 = t8440 * t3322;
    let t44234 = F::cast_from(6.0_f64) * t27229 * t9777;
    let t44236 = F::cast_from(4.0_f64) * t7324 * t10805;
    let t44238 = F::cast_from(4.0_f64) * t8862 * t9780;
    let t44239 = t29646 * t1052;
    (t44231, t44232, t44234, t44236, t44238, t44239)
}
