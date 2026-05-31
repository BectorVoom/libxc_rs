//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 570/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk570<F: Float>(t50: F, t1412: F, t954: F, t34: F, t52: F, t422: F, t532: F, t2464: F, t59: F, zeta_threshold: F) -> (F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t2465 = t1412 * t954;
    let t2468 = t52 * t34;
    let t2472 = piecewise3::<F>(t51, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2465 * t422 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2468 * t532);
    let t2474 = (t2464 + t2472) * t59;
    (t2465, t2468, t2474)
}
