//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 831/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk831<F: Float>(t50: F, t1412: F, t34: F, t422: F, t532: F, t1413: F, t1416: F, t2465: F, t2468: F, t39: F, t52: F, t6948: F, t59: F, t6947: F, zeta_threshold: F) -> (F, F) {
    let t51 = t50 <= zeta_threshold;
    let t6951 = t1412 * t34;
    let t6952 = t532 * t422;
    let t6962 = piecewise3::<F>(t51, F::new(0.0), -F::new(8.0) / F::new(27.0) * t6948 * t1413 - F::new(16.0) / F::new(9.0) * t6951 * t6952 + F::new(4.0) / F::new(9.0) * t2465 * t1416 - F::new(8.0) / F::new(3.0) * t52 * t532 + F::new(8.0) * t2468 * t39);
    let t6964 = (t6947 + t6962) * t59;
    (t6952, t6964)
}
