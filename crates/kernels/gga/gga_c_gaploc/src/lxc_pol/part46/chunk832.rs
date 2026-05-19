//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 832/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk832<F: Float>(t10525: F, t10526: F, t41749: F, t40106: F, t40109: F, t1445: F, t3085: F, t574: F, t7980: F, t3149: F, t8072: F, t12792: F, t555: F) -> (F, F, F, F, F, F) {
    let t41752 = F::cast_from(0.42900587942220512002e1_f64) * t10525 * t10526 * t41749;
    let t41753 = F::cast_from(0.29792074959875355558e-1_f64) * t40106;
    let t41754 = F::cast_from(0.59584149919750711116e-1_f64) * t40109;
    let t41759 = F::cast_from(0.92023022289409799224e1_f64) * t574 * t1445 * t7980 * t3085;
    let t41761 = F::cast_from(0.35750489951850426669e0_f64) * t3149 * t8072;
    let t41762 = t555 * t12792;
    (t41752, t41753, t41754, t41759, t41761, t41762)
}
