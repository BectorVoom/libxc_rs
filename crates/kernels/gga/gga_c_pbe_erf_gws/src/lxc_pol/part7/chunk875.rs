//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 875/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk875<F: Float>(t17799: F, t185: F, t5274: F, t582: F, t17771: F, t17773: F, t17778: F, t17783: F, t17785: F, t17790: F, t17794: F, t17796: F, t17798: F, t211: F, t5112: F, t5516: F, t583: F) -> (F, F, F, F, F) {
    let t17800 = 32.0 / 15.0 * t17799;
    let t17802 = t185 * t582 * t5274;
    let t17803 = 16.0 / 45.0 * t17802;
    let t17804 = -t17771 - t17773 - t17778 - t17783 + t17785 + t17790 + t17794 - t17796 - t17798 + t17800 - t17803;
    let t17806 = t211 * t582 * t5112;
    let t17807 = 32.0 / 15.0 * t17806;
    let t17808 = t5516 * t583;
    (t17800, t17803, t17804, t17807, t17808)
}
