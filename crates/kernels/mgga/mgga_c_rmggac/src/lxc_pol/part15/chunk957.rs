//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 957/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk957<F: Float>(t39103: F, t9222: F, t40323: F, t40313: F, t236: F, t615: F, t1981: F, t41799: F, t676: F, t46832: F, t7473: F, t7478: F, t40702: F, t8571: F, t40081: F, t46434: F, t7198: F) -> (F, F, F, F, F, F, F, F) {
    let t48027 = t9222 * t39103;
    let t48029 = t9222 * t40323;
    let t48031 = t9222 * t40313;
    let t48033 = t236 * t615;
    let t48036 = t41799 * t1981 * t676 * t48033;
    let t48038 = t46832 * t7473;
    let t48039 = t48038 * t7478;
    let t48041 = t8571 * t40702;
    let t48043 = t8571 * t40081;
    let t48047 = t7198 * t46434;
    (t48027, t48029, t48031, t48036, t48039, t48041, t48043, t48047)
}
