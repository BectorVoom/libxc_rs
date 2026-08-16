//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1000/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1000<F: Float>(t3142: F, t8967: F, t3172: F, t6484: F, t2206: F, t3195: F, t8574: F, t858: F, t886: F, t884: F, t1114: F, t6677: F) -> (F, F, F, F, F, F) {
    let t8969 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t8967 * t3142;
    let t8971 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6484 * t3172;
    let t8973 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2206 * t3195;
    let t8975 = t886 * t858 * t8574;
    let t8977 = t884 * t8975 / F::cast_from(48.0_f64);
    let t8978 = t1114 * t6677;
    (t8969, t8971, t8973, t8975, t8977, t8978)
}
