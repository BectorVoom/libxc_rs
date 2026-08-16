//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 820/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk820<F: Float>(t3679: F, t7785: F, t1354: F, t7710: F, t443: F, t8102: F, t7706: F, t3973: F, t8044: F, t1309: F, t6157: F, t6171: F) -> (F, F, F, F, F, F) {
    let t25894 = t7785 * t3679;
    let t25921 = t1354 * t7710;
    let t25925 = t443 * t8102;
    let t25947 = t1354 * t7706;
    let t25980 = t3973 * t8044;
    let t25981 = t1309 * t25980;
    let t25985 = t6157 * t6171;
    (t25894, t25921, t25925, t25947, t25981, t25985)
}
