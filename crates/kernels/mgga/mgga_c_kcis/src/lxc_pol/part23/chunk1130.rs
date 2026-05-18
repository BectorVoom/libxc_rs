//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1130/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1130<F: Float>(t2718: F, t2724: F, t873: F, t8913: F, t2727: F, t206: F, t220: F, t8942: F, t870: F, t8943: F, t687: F, t8747: F) -> (F, F, F, F, F, F) {
    let t36533 = t2718 * t2724;
    let t36543 = t8913 * t873;
    let t36901 = t2727 * t2727;
    let t36902 = F::new(1.0) / t36901;
    let t36908 = t206 / t8942 / t220;
    let t36936 = t870 * t8943;
    let t36951 = t8747 * t687;
    (t36533, t36543, t36902, t36908, t36936, t36951)
}
