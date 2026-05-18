//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 520/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk520<F: Float>(t221: F, t3493: F, t2747: F, t1010: F, t2615: F, t1891: F, t3351: F, t642: F, t639: F, t1896: F, t3342: F, t590: F) -> (F, F, F, F, F, F, F, F) {
    let t3495 = F::new(4.0) / F::new(15.0) * t3493 * t221;
    let t3496 = F::new(8.0) / F::new(45.0) * t2747;
    let t3498 = F::new(8.0) / F::new(45.0) * t2615 * t1010;
    let t3499 = t1891 * t3351;
    let t3500 = t642 * t3499;
    let t3502 = F::new(8.0) / F::new(45.0) * t639 * t3500;
    let t3503 = t1896 * t3342;
    let t3504 = t590 * t3503;
    (t3495, t3496, t3498, t3499, t3500, t3502, t3503, t3504)
}
