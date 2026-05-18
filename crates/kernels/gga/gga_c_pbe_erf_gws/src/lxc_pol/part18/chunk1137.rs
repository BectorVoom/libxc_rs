//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1137/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1137<F: Float>(t13792: F, t14469: F, t338: F, t4183: F, t892: F, t1115: F, t13772: F, t13809: F, t13939: F, t14437: F, t14444: F, t14448: F, t14452: F, t14457: F, t14460: F, t14464: F, t14467: F, t2408: F, t3066: F, t335: F, t827: F) -> (F, F) {
    let t14470 = t13792 * t14469;
    let t14473 = t338 * t892 * t4183;
    let t14477 = -t1115 * t13939 / F::new(96.0) - t827 * t14437 / F::new(96.0) - t1115 * t13772 / F::new(96.0) + t14444 / F::new(3072.0) + t2408 * t14448 / F::new(48.0) + t3066 * t14452 / F::new(48.0) + t14457 / F::new(768.0) + t2408 * t14460 / F::new(48.0) - t14464 / F::new(48.0) - t14467 / F::new(48.0) - t14470 / F::new(48.0) - t335 * t14473 / F::new(96.0) - F::new(7.0) / F::new(2304.0) * t13809;
    (t14473, t14477)
}
