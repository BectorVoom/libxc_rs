//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 893/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk893<F: Float>(t43093: F, t13194: F, t1841: F, t13200: F, t13182: F, t29439: F, t1897: F, t3270: F, t8942: F, t1022: F, t3234: F) -> (F, F, F, F, F, F) {
    let t43094 = F::cast_from(0.64087718584518535698e-3_f64) * t43093;
    let t43095 = t1841 * t13194;
    let t43098 = t1841 * t13200;
    let t43099 = F::cast_from(0.2563508743380741428e-2_f64) * t43098;
    let t43100 = t29439 * t13182;
    let t43101 = F::cast_from(0.64087718584518535698e-3_f64) * t43100;
    let t43106 = F::cast_from(0.76905262301422242837e-2_f64) * t1897 * t3270 * t8942;
    let t43107 = t3234 * t1022;
    (t43094, t43095, t43099, t43101, t43106, t43107)
}
