//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1217/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1217<F: Float>(t2142: F, t6612: F, t2083: F, t2084: F, t21570: F, t21577: F, t21580: F, t21581: F, t21586: F, t21594: F, t21596: F, t21600: F, t21601: F, t21605: F, t2253: F, t2312: F, t2343: F, t3257: F, t6195: F, t6275: F, t821: F, t904: F, t9343: F) -> (F, F) {
    let t21607 = t6612 * t2142;
    let t21608 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t21607;
    let t21609 = -t6275 * t904 * t821 * t2083 * t21570 / F::cast_from(16.0_f64) + t21577 + t21580 - t2253 * t3257 * t2084 * t21581 / F::cast_from(64.0_f64) + t2312 * t3257 * t6195 * t21586 / F::cast_from(16.0_f64) - t21594 + t21596 - t21600 - F::cast_from(5.0_f64) / F::cast_from(32.0_f64) * t2343 * t9343 * t21601 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t21605 - t21608;
    (t21608, t21609)
}
