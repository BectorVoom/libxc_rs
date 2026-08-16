//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 629/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk629<F: Float>(t2928: F, t3024: F, t312: F, t1267: F, t1271: F, t1394: F, t1398: F, t1446: F, t2098: F, t2510: F, t2511: F, t2512: F, t2514: F, t2516: F, t2517: F, t2842: F, t2844: F, t2846: F) -> (F, F) {
    let t3025 = t2928 + t3024;
    let t3026 = t3025 * t312;
    let t3027 = -t2510 - t1271 - t2511 + t1446 - t2512 + t2514 + t2516 - t1267 + t2098 - t1394 - t1398 - t2517 - t3026 - t2842 - t2844 + t2846;
    (t3025, t3027)
}
