//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1254/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1254<F: Float>(t35463: F, t35466: F, t35471: F, t35475: F, t35478: F, t35480: F, t35482: F, t35485: F, t35489: F, t35493: F, t35495: F, t35500: F, t35503: F) -> F {
    let t35505 = -F::new(0.18103800586153667463e-6) * t35463 - F::new(0.18103800586153667463e-6) * t35466 - F::new(0.39602063782211147576e-7) * t35471 + F::new(0.23761238269326688546e-5) * t35475 + F::new(0.23761238269326688546e-5) * t35478 - F::new(0.2530696388073708253e-5) * t35480 + F::new(0.21121100683845945374e-5) * t35482 - F::new(0.22776267492663374277e-4) * t35485 - F::new(0.2530696388073708253e-5) * t35489 + F::new(0.2530696388073708253e-5) * t35493 + F::new(0.12653481940368541265e-5) * t35495 - F::new(0.43449121406768801912e-4) * t35500 + F::new(0.2530696388073708253e-5) * t35503;
    t35505
}
