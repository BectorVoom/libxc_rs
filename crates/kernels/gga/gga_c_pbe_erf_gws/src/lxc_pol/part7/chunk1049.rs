//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1049/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1049<F: Float>(t20974: F, t20975: F, t20977: F, t20978: F, t20981: F, t20982: F, t20984: F, t20989: F, t369: F, t6084: F, t2100: F, t931: F, t2298: F, t814: F, t322: F, t6382: F) -> (F, F, F, F, F) {
    let t20992 = t20974 + t20975 + t20977 + t20978 + t20981 + t20982 + t20984 + t20989;
    let t20995 = t6084 * t369;
    let t20998 = t2100 * t931;
    let t21003 = t814 * t2298;
    let t21010 = t322 * t6382;
    (t20992, t20995, t20998, t21003, t21010)
}
