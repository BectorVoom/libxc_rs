//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 858/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk858<F: Float>(t2558: F, t33360: F, t9647: F, t13194: F, t1841: F, t13200: F, t13182: F, t29439: F, t13179: F, t7137: F, t1897: F, t3270: F, t8942: F, t1022: F, t3234: F) -> (F, F, F, F, F, F, F) {
    let t43093 = t9647 * t33360 * t2558;
    let t43094 = 0.64087718584518535698e-3 * t43093;
    let t43095 = t1841 * t13194;
    let t43096 = 0.17090058289204942852e-2 * t43095;
    let t43098 = t1841 * t13200;
    let t43099 = 0.2563508743380741428e-2 * t43098;
    let t43100 = t29439 * t13182;
    let t43101 = 0.64087718584518535698e-3 * t43100;
    let t43102 = t7137 * t13179;
    let t43106 = 0.76905262301422242837e-2 * t1897 * t3270 * t8942;
    let t43107 = t3234 * t1022;
    (t43094, t43096, t43099, t43101, t43102, t43106, t43107)
}
