//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1084/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1084<F: Float>(t4360: F, t994: F, t11333: F, t11395: F, t11400: F, t11416: F, t11421: F, t11425: F, t1416: F, t2595: F, t3544: F, t3564: F, t3582: F, t3601: F, t372: F, t4333: F, t4376: F, t7254: F, t7360: F, t7421: F, t9458: F, t9496: F, t9511: F, t977: F, t987: F) -> (F, F) {
    let t11432 = t4360 * t994;
    let t11435 = 0.5848223622634646207e0 * t987 * t11395 + 0.17315859105681463759e2 * t7254 * t4376 + 1.0 * t11400 * t977 + 2.0 * t9496 * t1416 + 2.0 * t3544 * t3564 - 2.0 * t7421 * t4333 - 0.19751673498613801407e-1 * t11333 + t11416 - 0.310907e-1 * t11421 * t372 + 0.2069040516770936012e4 * t7360 * t11425 - 0.23392894490538584828e1 * t9511 * t3582 + 0.34631718211362927517e2 * t9458 * t3601 + 0.35089341735807877242e1 * t2595 * t11432;
    (t11432, t11435)
}
