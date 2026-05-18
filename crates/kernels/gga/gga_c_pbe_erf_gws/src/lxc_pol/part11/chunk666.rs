//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 666/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk666<F: Float>(t267: F, t7114: F, t1791: F, t641: F, t1044: F, t1018: F, t1672: F, t185: F, t2789: F, t586: F) -> (F, F, F, F, F, F) {
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7117 = t7116 * t1044;
    let t7121 = t1672 * t1018;
    let t7122 = t185 * t7121;
    let t7130 = t2789 * t586;
    (t7115, t7116, t7117, t7121, t7122, t7130)
}
