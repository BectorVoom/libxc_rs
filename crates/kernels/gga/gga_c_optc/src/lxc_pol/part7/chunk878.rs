//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 878/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk878<F: Float>(t8428: F, t8950: F, t6548: F, t894: F, t1136: F, t6554: F, t464: F, t8912: F, t8914: F, t935: F, t438: F, t450: F, t465: F, t7448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8951 = t8950 * t8428;
    let t8952 = t8951 * t6548;
    let t8953 = t894 * t8952;
    let t8956 = t1136 * t6554;
    let t8957 = t894 * t8956;
    let t8960 = t464 * t8912;
    let t8961 = t8914 * t935;
    let t8962 = t8961 * t438;
    let t8963 = t450 * t8962;
    let t8966 = t465 * t7448;
    (t8951, t8952, t8953, t8956, t8957, t8960, t8962, t8963, t8966)
}
