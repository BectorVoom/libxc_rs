//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1139/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1139<F: Float>(t41524: F, t41562: F, t41570: F, t41573: F, t12651: F, t2615: F, t47391: F, t5008: F, t587: F, t590: F, t12572: F, t18309: F, t18311: F, t18315: F, t18318: F, t34538: F) -> (F, F, F, F, F, F, F, F) {
    let t48130 = F::new(64.0) / F::new(45.0) * t41524;
    let t48132 = F::new(32.0) / F::new(45.0) * t41562;
    let t48133 = F::new(32.0) / F::new(135.0) * t41570;
    let t48134 = F::new(256.0) / F::new(243.0) * t41573;
    let t48136 = F::new(16.0) / F::new(5.0) * t2615 * t12651;
    let t48140 = F::new(32.0) / F::new(15.0) * t587 * t590 * t5008 * t47391;
    let t48142 = F::new(32.0) / F::new(15.0) * t2615 * t12572;
    let t48143 = t18309 + t18311 - t18315 - t18318 + t48130 - F::new(4.0) / F::new(9.0) * t34538 + t48132 + t48133 + t48134 + t48136 - t48140 + t48142;
    (t48130, t48132, t48133, t48134, t48136, t48140, t48142, t48143)
}
