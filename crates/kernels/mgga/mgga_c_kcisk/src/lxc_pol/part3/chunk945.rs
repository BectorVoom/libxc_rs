//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 945/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk945<F: Float>(t3866: F, t970: F, t3870: F, t12925: F, t1398: F, t12831: F, t457: F, t3875: F, t960: F, t3878: F, t1375: F, t1471: F) -> (F, F, F, F, F, F, F, F) {
    let t13991 = t970 * t3866;
    let t13993 = t970 * t3870;
    let t13995 = t1398 * t12925;
    let t13998 = t457 * t12831;
    let t14001 = t960 * t3875;
    let t14003 = t960 * t3878;
    let t14005 = t1375 * t12925;
    let t14008 = t1471 * t12831;
    (t13991, t13993, t13995, t13998, t14001, t14003, t14005, t14008)
}
