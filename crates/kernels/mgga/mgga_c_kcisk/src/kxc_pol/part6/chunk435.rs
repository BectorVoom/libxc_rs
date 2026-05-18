//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 435/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk435<F: Float>(t190: F, t3232: F, t207: F, t1031: F, t981: F, t1036: F, t1032: F, t1039: F, t205: F, t3137: F, t3139: F, t1001: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3233 = t3232 * t190;
    let t3234 = t3233 * t207;
    let t3236 = t1031 * t981;
    let t3237 = t3236 * t1036;
    let t3239 = t1032 * t1039;
    let t3241 = t205 * t3137;
    let t3242 = t207 * t3139;
    let t3243 = t3241 * t3242;
    let t3245 = t1039 * t1001;
    (t3233, t3234, t3236, t3237, t3239, t3241, t3242, t3243, t3245)
}
