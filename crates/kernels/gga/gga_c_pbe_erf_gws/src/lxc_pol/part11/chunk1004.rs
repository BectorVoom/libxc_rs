//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1004/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1004<F: Float>(t47391: F, t5008: F, t587: F, t590: F, t12572: F, t2615: F, t18309: F, t18311: F, t18315: F, t18318: F, t34538: F, t48130: F, t48132: F, t48133: F, t48134: F, t48136: F) -> (F, F, F) {
    let t48140 = 32.0 / 15.0 * t587 * t590 * t5008 * t47391;
    let t48142 = 32.0 / 15.0 * t2615 * t12572;
    let t48143 = t18309 + t18311 - t18315 - t18318 + t48130 - 4.0 / 9.0 * t34538 + t48132 + t48133 + t48134 + t48136 - t48140 + t48142;
    (t48140, t48142, t48143)
}
