//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 773/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk773<F: Float>(t43: F, t50: F, t1402: F, t34: F, t418: F, t532: F, t1403: F, t1407: F, t2457: F, t2460: F, t39: F, t47: F, t6933: F, t4767: F, t954: F, t1412: F, t422: F, t1413: F, t1416: F, t2465: F, t2468: F, t52: F, zeta_threshold: F) -> (F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t6936 = t1402 * t34;
    let t6937 = t532 * t418;
    let t6947 = piecewise3(t44, 0.0, -8.0 / 27.0 * t6933 * t1403 + 16.0 / 9.0 * t6936 * t6937 + 4.0 / 9.0 * t2457 * t1407 + 8.0 / 3.0 * t47 * t532 - 8.0 * t2460 * t39);
    let t6948 = t4767 * t954;
    let t6951 = t1412 * t34;
    let t6952 = t532 * t422;
    let t6962 = piecewise3(t51, 0.0, -8.0 / 27.0 * t6948 * t1413 - 16.0 / 9.0 * t6951 * t6952 + 4.0 / 9.0 * t2465 * t1416 - 8.0 / 3.0 * t52 * t532 + 8.0 * t2468 * t39);
    (t6937, t6947, t6952, t6962)
}
