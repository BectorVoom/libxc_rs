//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 740/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk740<F: Float>(t443: F, t8102: F, t1354: F, t7706: F, t3973: F, t8044: F, t1309: F, t6157: F, t6171: F, t1308: F, t8021: F, t13485: F, t8036: F, t3935: F, t8040: F, t13917: F, t8032: F) -> (F, F, F, F, F, F, F, F) {
    let t25925 = t443 * t8102;
    let t25947 = t1354 * t7706;
    let t25980 = t3973 * t8044;
    let t25981 = t1309 * t25980;
    let t25985 = t6157 * t6171;
    let t26008 = t8021 * t1308;
    let t26064 = t13485 * t8036;
    let t26065 = t3935 * t26064;
    let t26074 = t3973 * t8040;
    let t26075 = t1309 * t26074;
    let t26085 = t13917 * t8032;
    (t25925, t25947, t25981, t25985, t26008, t26065, t26075, t26085)
}
