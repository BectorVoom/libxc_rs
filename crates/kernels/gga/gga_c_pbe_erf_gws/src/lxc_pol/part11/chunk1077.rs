//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1077/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1077<F: Float>(t18527: F, t18529: F, t18556: F, t18562: F, t48478: F, t48479: F, t48480: F, t48481: F, t48482: F, t48483: F, t48484: F, t48485: F, t48486: F, t48488: F, t18567: F, t18571: F, t18574: F, t18577: F, t18580: F, t18587: F, t18594: F, t18599: F, t18604: F, t18607: F, t18610: F, t18619: F, t18624: F, t48489: F) -> (F, F) {
    let t49419 = -t48478 + t48479 + t18527 - t18529 - t48480 - t48481 - t48482 - t48483 - t48484 + t48485 - t48486 - t18556 - t18562 + t48488;
    let t49420 = t18567 + t18571 - t18574 - t48489 + t18577 + t18580 + t18587 + t18594 + t18599 - t18604 - t18607 - t18610 - t18619 - t18624;
    (t49419, t49420)
}
