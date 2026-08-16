//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1324/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1324<F: Float>(t14911: F, t2367: F, t353: F, t4228: F, t4386: F, t810: F, t53625: F, t1115: F, t14311: F, t14327: F, t14888: F, t14894: F, t20113: F, t22134: F, t29751: F, t3040: F, t3207: F, t4083: F, t51526: F, t52345: F, t52480: F, t53599: F, t53601: F, t53623: F, t6793: F, t8634: F) -> F {
    let t55279 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14911;
    let t55284 = t4386 * t353 * t4228 * t810;
    let t55290 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53625;
    let t55294 = -t3207 * t29751 * t14894 / F::cast_from(8.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t52345 + t53599 / F::cast_from(12.0_f64) + t53601 / F::cast_from(24.0_f64) - t8634 * t4083 / F::cast_from(48.0_f64) - t3040 * t14311 / F::cast_from(48.0_f64) - t3040 * t14327 / F::cast_from(48.0_f64) + t55279 - t1115 * t52480 / F::cast_from(96.0_f64) + t6793 * t55284 / F::cast_from(24.0_f64) + t20113 * t14888 / F::cast_from(48.0_f64) - t53623 / F::cast_from(768.0_f64) + t55290 - t22134 * t4083 / F::cast_from(96.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51526;
    t55294
}
