//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1131/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1131<F: Float>(t13792: F, t14469: F, t338: F, t4183: F, t892: F, t1115: F, t13772: F, t13809: F, t13939: F, t14437: F, t14444: F, t14448: F, t14452: F, t14457: F, t14460: F, t14464: F, t14467: F, t2408: F, t3066: F, t335: F, t827: F) -> (F, F) {
    let t14470 = t13792 * t14469;
    let t14473 = t338 * t892 * t4183;
    let t14477 = -t1115 * t13939 / F::cast_from(96.0_f64) - t827 * t14437 / F::cast_from(96.0_f64) - t1115 * t13772 / F::cast_from(96.0_f64) + t14444 / F::cast_from(3072.0_f64) + t2408 * t14448 / F::cast_from(48.0_f64) + t3066 * t14452 / F::cast_from(48.0_f64) + t14457 / F::cast_from(768.0_f64) + t2408 * t14460 / F::cast_from(48.0_f64) - t14464 / F::cast_from(48.0_f64) - t14467 / F::cast_from(48.0_f64) - t14470 / F::cast_from(48.0_f64) - t335 * t14473 / F::cast_from(96.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t13809;
    (t14473, t14477)
}
