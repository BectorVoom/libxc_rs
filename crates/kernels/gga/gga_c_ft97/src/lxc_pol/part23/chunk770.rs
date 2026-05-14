//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 770/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk770<F: Float>(t5410: F, t8392: F, t1212: F, t2842: F, t4181: F, t15460: F, t5415: F, t18497: F, t4140: F, t4139: F, t312: F, t5225: F, t684: F, t10492: F, t15370: F, t4176: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19504 = t8392 * t5410;
    let t19506 = t2842 * t1212;
    let t19507 = t19506 * t4181;
    let t19508 = t15460 * t19507;
    let t19511 = t8392 * t5415;
    let t19513 = t4140 * t18497;
    let t19514 = t4139 * t19513;
    let t19517 = t312 * t5225;
    let t19518 = t19517 * t684;
    let t19519 = t10492 * t19518;
    let t19522 = t15370 * t4176;
    (t19504, t19507, t19508, t19511, t19513, t19514, t19518, t19519, t19522)
}
