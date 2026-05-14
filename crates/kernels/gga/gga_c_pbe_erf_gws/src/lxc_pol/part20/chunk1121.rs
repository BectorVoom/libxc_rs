//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1121/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1121<F: Float>(t1113: F, t2118: F, t3028: F, t3972: F, t3975: F, t14767: F, t3052: F, t14657: F, t8602: F, t15149: F, t3038: F, t8716: F, t1134: F, t13917: F, t53799: F, t824: F, t938: F) -> (F, F, F, F, F, F) {
    let t56166 = t3972 * t3975 * t1113 * t2118 * t3028;
    let t56168 = t14767 * t3052;
    let t56170 = t14657 * t8602;
    let t56174 = t3972 * t3975 * t3038 * t15149;
    let t56176 = t14657 * t8716;
    let t56181 = t13917 * t53799 * t824 * t1134 * t938;
    (t56166, t56168, t56170, t56174, t56176, t56181)
}
