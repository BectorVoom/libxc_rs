//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1165/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1165<F: Float>(t22606: F, t22609: F, t18562: F, t18567: F, t18571: F, t18574: F, t18577: F, t18580: F, t18587: F, t18594: F, t18599: F, t18604: F, t18607: F) -> (F, F, F) {
    let t48488 = F::new(4.0) * t22606;
    let t48489 = F::new(48.0) * t22609;
    let t48490 = -t18562 + t48488 + t18567 + t18571 - t18574 - t48489 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607;
    (t48488, t48489, t48490)
}
