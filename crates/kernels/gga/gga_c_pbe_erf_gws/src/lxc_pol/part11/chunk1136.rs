//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1136/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1136<F: Float>(t13070: F, t13073: F, t13088: F, t13702: F, t13704: F, t16329: F, t16331: F, t16334: F, t16335: F, t16336: F, t16337: F, t16338: F, t16340: F, t48936: F, t48948: F, t48950: F, t48957: F, t50751: F, t50759: F, t50767: F, t50771: F, t6906: F, t6968: F, t7: F, t9763: F) -> (F,) {
    let t50784 = t7 * (t48936 + t48948 + t48950 + t48957 + t50751 + t50759 + t50767 + t50771) - 4.0 * t13070 - 0.21973866044103791929e-2 * t9763 + 0.82152657680133333336e1 * t6906 + t16329 - t16331 - t16334 + t16335 - t16336 - 12.0 * t13073 - t16337 + t16338 + t16340 + 12.0 * t13088 + 4.0 * t13702 + 36.0 * t13704 + 0.13012297059337829058e0 * t6968;
    (t50784,)
}
