//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1082/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1082<F: Float>(t1006: F, t125: F, t1552: F, t1954: F, t200: F, t11227: F, t35491: F, t8291: F, t35463: F, t35466: F, t35471: F, t35475: F, t35478: F, t35480: F, t35482: F, t35485: F, t35489: F, t35493: F, t35495: F) -> (F,) {
    let t35500 = t1006 * t125 * t1552 * t200 * t1954;
    let t35503 = t35491 * t11227 * t8291;
    let t35505 = -0.18103800586153667463e-6 * t35463 - 0.18103800586153667463e-6 * t35466 - 0.39602063782211147576e-7 * t35471 + 0.23761238269326688546e-5 * t35475 + 0.23761238269326688546e-5 * t35478 - 0.2530696388073708253e-5 * t35480 + 0.21121100683845945374e-5 * t35482 - 0.22776267492663374277e-4 * t35485 - 0.2530696388073708253e-5 * t35489 + 0.2530696388073708253e-5 * t35493 + 0.12653481940368541265e-5 * t35495 - 0.43449121406768801912e-4 * t35500 + 0.2530696388073708253e-5 * t35503;
    (t35505,)
}
