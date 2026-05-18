//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 874/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk874<F: Float>(t13505: F, t13514: F, t13520: F, t13522: F, t13527: F, t13529: F, t13538: F, t13567: F, t13569: F, t13575: F, t13582: F, t13583: F, t13602: F, t6597: F) -> F {
    let t13675 = -t6597 - t13505 - t13514 + t13520 - t13522 + t13527 + t13529 + t13538 - t13567 - t13569 - t13575 + t13582 + t13583 - t13602;
    t13675
}
