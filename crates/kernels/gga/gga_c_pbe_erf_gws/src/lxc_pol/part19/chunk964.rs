//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 964/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk964<F: Float>(t3479: F, t636: F, t3493: F, t3397: F, t577: F, t184: F, t199: F, t7778: F, t3399: F, t612: F, t1004: F, t562: F) -> (F, F, F, F, F, F) {
    let t10887 = t3479 * t636;
    let t10888 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t10887;
    let t10889 = t3493 * t636;
    let t10890 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10889;
    let t10891 = t3397 * t577;
    let t10892 = t10891 * t184;
    let t10894 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t10892 * t199;
    let t10895 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7778;
    let t10897 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t3399 * t612;
    let t10898 = t562 * t1004;
    (t10888, t10890, t10894, t10895, t10897, t10898)
}
