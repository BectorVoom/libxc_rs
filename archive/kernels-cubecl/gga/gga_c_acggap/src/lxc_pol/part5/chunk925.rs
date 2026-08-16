//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 925/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk925<F: Float>(t14173: F, t3393: F, t1172: F, t3077: F, t3198: F, t1165: F, t3194: F, t3196: F, t991: F, t3073: F, t3360: F, t3371: F) -> (F, F, F, F, F, F) {
    let t14174 = t14173 * t3393;
    let t14176 = t3077 * t1172;
    let t14177 = t14176 * t3198;
    let t14181 = t3194 * t1165 * t991 * t3196;
    let t14187 = t3073 * t1172;
    let t14220 = t3360 * t3371;
    (t14174, t14176, t14177, t14181, t14187, t14220)
}
