//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 751/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk751<F: Float>(t50: F, t12350: F, t12355: F, t2465: F, t3354: F, t4767: F, t52: F, t12349: F, t59: F, zeta_threshold: F) -> F {
    let t51 = t50 <= zeta_threshold;
    let t12359 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4767 * t12350 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2465 * t3354 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t52 * t12355);
    let t12361 = (t12349 + t12359) * t59;
    t12361
}
