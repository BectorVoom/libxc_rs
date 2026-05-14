//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 792/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk792<F: Float>(t3795: F, t3868: F, t3880: F, t3881: F, t5469: F, t5472: F, t5475: F, t5479: F, t5514: F, t5516: F, t5557: F, t5559: F, t5562: F, t5565: F, t5568: F, t5571: F) -> (F,) {
    let t5573 = -0.9494625e0 * t5514 + 0.1898925e1 * t5516 + t3868 + 0.99655555555555555557e-1 * t3795 + 0.99655555555555555557e-1 * t5469 - 0.19931111111111111111e0 * t5472 + 0.59793333333333333334e0 * t5475 + 0.59793333333333333334e0 * t5479 + 0.15358125e0 * t5557 + 0.3071625e0 * t5559 + t3880 + 0.54771111111111111111e-1 * t3881 + 0.54771111111111111111e-1 * t5562 - 0.27385555555555555556e-1 * t5565 + 0.16431333333333333333e0 * t5568 + 0.16431333333333333333e0 * t5571;
    (t5573,)
}
