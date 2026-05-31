//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 971/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk971<F: Float>(t10968: F, t184: F, t564: F, t3450: F, t582: F, t561: F, t5513: F, t1006: F, t2786: F, t3425: F, t610: F, t1827: F) -> (F, F, F, F, F) {
    let t10969 = t10968 * t184;
    let t10971 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t10969 * t564;
    let t10972 = t582 * t3450;
    let t10973 = t561 * t10972;
    let t10974 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t10973;
    let t10975 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t5513;
    let t10977 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1006 * t2786;
    let t10978 = t3425 * t610;
    let t10979 = t1827 * t10978;
    (t10971, t10974, t10975, t10977, t10979)
}
