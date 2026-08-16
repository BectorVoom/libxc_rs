//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1161/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1161<F: Float>(t16481: F, t16486: F, t16489: F, t16493: F, t16497: F, t16513: F, t16517: F, t16526: F, t16531: F, t19693: F, t19694: F, t19695: F, t19696: F, t19697: F, t19703: F, t19711: F, t19712: F) -> F {
    let t20318 = t16481 - t16486 - t16489 - t16493 + t16497 - t19693 - t19694 + t19695 - t19696 + t19697 - t16513 + t16517 + t19703 - t19711 + t16526 + t16531 + t19712;
    t20318
}
