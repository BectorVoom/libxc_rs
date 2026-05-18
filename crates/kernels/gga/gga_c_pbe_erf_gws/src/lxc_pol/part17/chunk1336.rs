//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1336/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1336<F: Float>(t13953: F, t14781: F, t14001: F, t3062: F, t14772: F, t13888: F, t14792: F, t2408: F, t29751: F, t3066: F, t51928: F, t51930: F, t54496: F, t54502: F, t54505: F, t54508: F, t54512: F, t54519: F, t54523: F, t54529: F, t9283: F, t9702: F) -> F {
    let t54531 = t13953 * t14781;
    let t54532 = F::new(7.0) / F::new(144.0) * t54531;
    let t54535 = t14001 * t3062;
    let t54536 = F::new(7.0) / F::new(72.0) * t54535;
    let t54537 = t14001 * t14772;
    let t54538 = F::new(7.0) / F::new(72.0) * t54537;
    let t54539 = -t54496 / F::new(24.0) - t54502 / F::new(768.0) + t54505 + t54508 / F::new(384.0) + t54512 / F::new(768.0) - t3066 * t29751 * t14792 / F::new(8.0) + t54519 / F::new(48.0) + t54523 / F::new(96.0) - t2408 * t9283 * t13888 * t9702 / F::new(12.0) + t54529 / F::new(48.0) + t54532 + F::new(7.0) / F::new(4608.0) * t51928 + F::new(7.0) / F::new(288.0) * t51930 - t54536 + t54538;
    t54539
}
