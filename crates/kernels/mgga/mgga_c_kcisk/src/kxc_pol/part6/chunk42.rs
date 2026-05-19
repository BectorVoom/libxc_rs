//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 42/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk42<F: Float>(t60: F, t6: F, t123: F, t121: F, t21: F, t2: F) -> (F, F, F, F, F, F) {
    let t124 = F::new(0.0) < t60;
    let t126 = piecewise3::<F>(t124, t60, -t60);
    let t127 = F::new(1.0) / t126;
    let t128 = t6 * t127;
    let t129 = t123 * t128;
    let t132 = F::new(1.0) + F::cast_from(0.53972366148531951642e-1_f64) * t121 * t129;
    let t133 = F::ln(t132);
    let t135 = F::new(1.0) + F::new(0.193e0) * t133;
    let t136 = F::new(1.0) / t135;
    let t138 = F::new(1.0) / t21;
    let t139 = t2 * t138;
    (t126, t129, t132, t135, t136, t139)
}
