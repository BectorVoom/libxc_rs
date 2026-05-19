//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 854/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk854<F: Float>(t229: F, t7007: F, t1721: F, t898: F, t5393: F, t2483: F, t5: F, t736: F, t41: F, t5366: F, t5373: F, t5378: F, t5384: F, t5392: F, t5401: F, t5405: F, t5409: F) -> F {
    let t7733 = t7007 * t229;
    let t7737 = t898 * t1721;
    let t7739 = F::new(48.0) * t5393;
    let t7741 = t2483 * t5;
    let t7743 = F::cast_from(0.10843581300301739842e-1_f64) * t7741 * t736;
    let t7744 = -t41 * t7733 + t5366 + F::new(0.3429168e0) * t5373 + F::cast_from(0.16008171603946666666e-1_f64) * t5378 + F::cast_from(0.65061487801810439052e-1_f64) * t7737 + t5384 - t5392 - t7739 - t5401 - t5405 + F::cast_from(0.84681398666666666666e-3_f64) * t5409 - t7743;
    t7744
}
