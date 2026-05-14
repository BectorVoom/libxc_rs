//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 419/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk419<F: Float>(t510: F, t513: F, t137: F, t512: F, t131: F, t520: F, t120: F, t133: F, t542: F, t1541: F, t1511: F, t1517: F, t1519: F, t1522: F, t1536: F, t1545: F, t1549: F) -> (F, F, F, F, F, F, F) {
    let t1572 = t510 * t513;
    let t1576 = 1.0 / t512 / t137;
    let t1577 = t131 * t1576;
    let t1578 = t520 * t520;
    let t1583 = 0.38316777777777777777e0 * t133 * t542 * t120;
    let t1584 = t133 * t1541;
    let t1590 = -t1511 + t1517 + t1519 + t1522 - t1536 + t1583 + 0.11495033333333333333e1 * t1584 + 0.5172765e1 * t133 * t1545 - 0.1724255e1 * t133 * t1549;
    (t1572, t1576, t1577, t1578, t1583, t1584, t1590)
}
