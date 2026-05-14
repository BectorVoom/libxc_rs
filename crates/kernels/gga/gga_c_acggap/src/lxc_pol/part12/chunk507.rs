//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 507/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk507<F: Float>(t130: F, t985: F, t138: F, t1046: F, t134: F, t347: F, t1049: F, t1065: F, t227: F, t8: F, t14: F, t2: F, t41: F, t135: F, t157: F, t360: F, t406: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3140 = t130 * t985;
    let t3141 = t3140 * t138;
    let t3142 = 70.0 / 27.0 * t3141;
    let t3143 = t1046 * t134;
    let t3144 = t3143 * t347;
    let t3146 = t1049 * t1065;
    let t3151 = 1.0 / t8 / t227;
    let t3152 = t130 * t3151;
    let t3153 = t3152 * t134;
    let t3157 = 1.0 / t14 / t2 / t41 / 48.0;
    let t3159 = t135 * t3157 * t2;
    let t3160 = t3153 * t3159;
    let t3161 = 5.0 / 6.0 * t3160;
    let t3169 = t360 * t406 * t157;
    (t3141, t3142, t3143, t3144, t3146, t3151, t3157, t3160, t3161, t3169)
}
