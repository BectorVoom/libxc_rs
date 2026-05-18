//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 836/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk836<F: Float>(t1928: F, t4134: F, t1981: F, t4121: F, t3728: F, t5634: F, t5758: F, t5417: F, t4135: F, t4169: F, t5877: F, t11670: F, t540: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16617 = t1928 * t4134;
    let t16622 = t1981 * t4121;
    let t16623 = t16622 * sigma2;
    let t16627 = t3728 * t5634;
    let t16628 = F::new(0.88437037037037037034e-2) * t16627;
    let t16629 = t3728 * t5758;
    let t16631 = t3728 * t5417;
    let t16632 = F::new(0.33163888888888888888e-2) * t16631;
    let t16633 = t4169 * t4135;
    let t16663 = t3728 * t5877;
    let t16690 = t11670 * t540;
    (t16617, t16622, t16623, t16627, t16628, t16629, t16631, t16632, t16633, t16663, t16690)
}
