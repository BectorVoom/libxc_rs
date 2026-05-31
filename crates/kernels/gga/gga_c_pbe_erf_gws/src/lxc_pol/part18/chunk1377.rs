//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1377/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1377<F: Float>(t1115: F, t11342: F, t13772: F, t3921: F, t4002: F, t51967: F, t54617: F, t54711: F, t55892: F, t57626: F, t57635: F, t57639: F, t57641: F, t57643: F, t57648: F, t57650: F, t57652: F, t57654: F) -> F {
    let t57656 = -t1115 * t54711 / F::cast_from(48.0_f64) - t57626 / F::cast_from(768.0_f64) - t11342 * t4002 / F::cast_from(96.0_f64) - t3921 * t13772 / F::cast_from(96.0_f64) - t57635 / F::cast_from(1536.0_f64) + t54617 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t51967 - t55892 - t57639 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t57641 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57643 - t57648 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t57650 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t57652 + t57654 / F::cast_from(24.0_f64);
    t57656
}
