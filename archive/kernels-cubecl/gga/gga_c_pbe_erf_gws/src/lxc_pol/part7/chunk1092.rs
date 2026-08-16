//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1092/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1092<F: Float>(t50: F, t19072: F, t19075: F, t19077: F, t19079: F, t19081: F, t19544: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t19551 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t19072 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t19075 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19077 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t19079 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t19081);
    let t19553 = t19544 / F::cast_from(2.0_f64) + t19551 / F::cast_from(2.0_f64);
    t19553
}
