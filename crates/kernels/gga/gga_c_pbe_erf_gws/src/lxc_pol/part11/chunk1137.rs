//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1137/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1137<F: Float>(t10017: F, t12363: F, t13708: F, t13711: F, t13714: F, t13717: F, t13721: F, t16345: F, t16349: F, t16350: F, t16351: F, t16353: F, t16354: F, t7986: F, t7988: F, t7990: F, t8520: F) -> (F,) {
    let t50796 = 24.0 * t13708 + 36.0 * t13711 + 72.0 * t13714 - 0.75926915593978166528e1 * t8520 + 72.0 * t13717 + 240.0 * t7986 + 12.0 * t10017 + 384.0 * t7988 - 96.0 * t7990 + 8.0 * t13721 + t16345 + t16349 + t16350 + t16351 + 4.0 * t12363 + t16353 - t16354;
    (t50796,)
}
