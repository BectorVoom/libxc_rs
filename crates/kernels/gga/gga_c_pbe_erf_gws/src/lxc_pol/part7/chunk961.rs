//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 961/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk961<F: Float>(t17802: F, t17771: F, t17773: F, t17778: F, t17783: F, t17785: F, t17790: F, t17794: F, t17796: F, t17798: F, t17800: F, t211: F, t5112: F, t582: F) -> (F, F, F) {
    let t17803 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t17802;
    let t17804 = -t17771 - t17773 - t17778 - t17783 + t17785 + t17790 + t17794 - t17796 - t17798 + t17800 - t17803;
    let t17806 = t211 * t582 * t5112;
    (t17803, t17804, t17806)
}
