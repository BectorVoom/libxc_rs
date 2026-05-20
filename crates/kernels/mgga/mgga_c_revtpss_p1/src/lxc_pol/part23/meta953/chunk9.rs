//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3172/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3172<F: Float>(t21213: F, t5357: F, t17401: F, t21166: F, t21259: F, t57126: F, t70378: F, t70382: F, t70394: F, t70403: F, t70405: F, t70411: F, t70427: F, t70432: F) -> F {
    let t83316 = t21213 * t5357;
    let t83322 = F::cast_from(0.42874018118069736972e-3_f64) * t70378 + F::cast_from(0.95275595817932748825e-3_f64) * t70382 + F::cast_from(0.28582678745379824648e-3_f64) * t70394 - t57126 + F::cast_from(0.30488190661738479624e-2_f64) * t70403 + F::cast_from(0.19055119163586549765e-3_f64) * t70405 + F::cast_from(0.17149607247227894789e-2_f64) * t70411 - F::cast_from(0.57165357490759649295e-3_f64) * t70427 - F::cast_from(0.28582678745379824648e-3_f64) * t70432 - F::new(11.0) / F::new(324.0) * t83316 - F::cast_from(0.12862205435420921092e-2_f64) * t17401 * t21166 - F::cast_from(0.12862205435420921092e-2_f64) * t17401 * t21259;
    t83322
}
