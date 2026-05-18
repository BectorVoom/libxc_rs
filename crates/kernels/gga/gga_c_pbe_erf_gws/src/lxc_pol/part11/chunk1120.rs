//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1120<F: Float>(t1010: F, t40493: F, t12440: F, t30630: F, t10848: F, t3527: F, t1006: F, t12703: F, t12576: F, t2612: F, t12560: F, t7130: F) -> (F, F, F, F, F, F) {
    let t47862 = F::new(16.0) / F::new(45.0) * t40493 * t1010;
    let t47864 = F::new(16.0) / F::new(5.0) * t30630 * t12440;
    let t47866 = F::new(8.0) / F::new(15.0) * t10848 * t3527;
    let t47868 = F::new(8.0) / F::new(15.0) * t1006 * t12703;
    let t47870 = F::new(32.0) / F::new(15.0) * t2612 * t12576;
    let t47872 = F::new(32.0) / F::new(5.0) * t7130 * t12560;
    (t47862, t47864, t47866, t47868, t47870, t47872)
}
