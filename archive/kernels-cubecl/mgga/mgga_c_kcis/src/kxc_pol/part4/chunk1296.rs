//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1296/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1296<F: Float>(t1395: F, t16618: F, t1464: F, t1981: F, t4121: F, t4125: F, t3728: F, t5634: F, t5758: F, t5417: F, t4135: F, t4169: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16619 = t1395 * t16618;
    let t16620 = t1464 * t16619;
    let t16622 = t1981 * t4121;
    let t16623 = t16622 * sigma2;
    let t16624 = t16623 * t4125;
    let t16625 = t1464 * t16624;
    let t16627 = t3728 * t5634;
    let t16628 = F::cast_from(0.88437037037037037034e-2_f64) * t16627;
    let t16629 = t3728 * t5758;
    let t16631 = t3728 * t5417;
    let t16632 = F::cast_from(0.33163888888888888888e-2_f64) * t16631;
    let t16633 = t4169 * t4135;
    (t16620, t16622, t16625, t16627, t16628, t16629, t16631, t16632, t16633)
}
