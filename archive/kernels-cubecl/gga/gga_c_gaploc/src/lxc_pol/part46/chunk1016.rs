//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1016/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1016<F: Float>(t1445: F, t25405: F, t3209: F, t5748: F, t13034: F, t15751: F, t10948: F, t9972: F, t41448: F, t41451: F, t41454: F, t41457: F) -> (F, F, F, F, F, F, F) {
    let t44142 = F::cast_from(0.27606906686822939767e2_f64) * t5748 * t1445 * t25405 * t3209;
    let t44144 = F::cast_from(0.27606906686822939767e2_f64) * t15751 * t13034;
    let t44145 = t10948 * t9972;
    let t44147 = F::cast_from(0.31952438294933958063e0_f64) * t41448;
    let t44148 = F::cast_from(0.89376224879626066674e-1_f64) * t41451;
    let t44149 = F::cast_from(0.59584149919750711116e-1_f64) * t41454;
    let t44150 = F::cast_from(0.15337170381568299871e1_f64) * t41457;
    (t44142, t44144, t44145, t44147, t44148, t44149, t44150)
}
