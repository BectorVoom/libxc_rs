//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1078/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1078<F: Float>(t16481: F, t16486: F, t16489: F, t16493: F, t16497: F, t16513: F, t16517: F, t16526: F, t16531: F, t19695: F, t19697: F, t28920: F, t28925: F, t28928: F, t28930: F, t28931: F, t28932: F) -> (F,) {
    let t29112 = t28920 + t16481 - t16486 - t16489 - t16493 + t16497 - t19695 - t28925 - t19697 - t16513 + t16517 + t28928 - t28930 + t16526 + t28931 + t16531 + t28932;
    (t29112,)
}
