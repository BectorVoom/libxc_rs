//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1177/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1177<F: Float>(t36901: F, t206: F, t220: F, t8942: F, t870: F, t8943: F, t687: F, t8747: F, t2388: F, t2379: F, t2385: F, t60: F, t81: F, t9260: F) -> (F, F, F, F, F, F, F) {
    let t36902 = F::new(1.0) / t36901;
    let t36908 = t206 / t8942 / t220;
    let t36936 = t870 * t8943;
    let t36951 = t8747 * t687;
    let t36957 = t2388 * t2388;
    let t36958 = F::new(1.0) / t36957;
    let t36962 = t2379 * t2385;
    let t37000 = t60 / t9260 / t81;
    (t36902, t36908, t36936, t36951, t36958, t36962, t37000)
}
