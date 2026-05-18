//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1218/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1218<F: Float>(t51414: F, t51458: F, t4116: F, t6854: F, t14369: F, t321: F, t14166: F, t2429: F, t1167: F, t2423: F, t3324: F, t810: F) -> (F, F, F, F, F, F, F) {
    let t52696 = F::new(595.0) / F::new(2592.0) * t51414;
    let t52715 = F::new(455.0) / F::new(648.0) * t51458;
    let t52751 = t4116 * t6854;
    let t52755 = t321 * t14369;
    let t52757 = t2429 * t14166;
    let t52763 = t1167 * t2423;
    let t52767 = t3324 * t810;
    (t52696, t52715, t52751, t52755, t52757, t52763, t52767)
}
