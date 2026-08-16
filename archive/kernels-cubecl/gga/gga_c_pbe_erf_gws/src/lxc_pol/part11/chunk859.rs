//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 859/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk859<F: Float>(t13496: F, t2168: F, t13220: F, t6384: F, t904: F, t11946: F, t11600: F, t3180: F, t13086: F, t933: F, t13125: F, t6472: F) -> (F, F, F, F, F, F) {
    let t13498 = t2168 * t13496 / F::cast_from(16.0_f64);
    let t13500 = t6384 * t904 * t13220;
    let t13503 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t11946;
    let t13505 = t11600 * t3180 / F::cast_from(16.0_f64);
    let t13507 = t933 * t904 * t13086;
    let t13510 = t13125 * t6472;
    (t13498, t13500, t13503, t13505, t13507, t13510)
}
