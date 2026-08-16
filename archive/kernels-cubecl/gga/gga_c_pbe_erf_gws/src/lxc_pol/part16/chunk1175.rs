//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1175/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1175<F: Float>(t26654: F, t938: F, t1161: F, t19631: F, t2182: F, t2501: F, t831: F, t8574: F, t2376: F, t9688: F, t810: F, t8749: F) -> (F, F, F, F, F, F) {
    let t26655 = t26654 * t938;
    let t26668 = t19631 * t1161;
    let t26768 = t2501 * t2182;
    let t26880 = t831 * t8574;
    let t26885 = t2376 * t9688;
    let t26933 = t8749 * t810;
    (t26655, t26668, t26768, t26880, t26885, t26933)
}
