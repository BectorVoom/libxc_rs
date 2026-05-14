//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 724/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk724<F: Float>(t1725: F, t582: F, t211: F, t1879: F, t1882: F, t1748: F, t202: F, t184: F, t1871: F, t561: F, t1680: F, t583: F, t1778: F, t633: F, t198: F, t2735: F) -> (F, F, F, F, F, F, F) {
    let t5322 = t582 * t1725;
    let t5323 = t211 * t5322;
    let t5338 = t1879 * t1882;
    let t5342 = t202 * t1748;
    let t5343 = t5342 * t184;
    let t5346 = t582 * t1871;
    let t5347 = t561 * t5346;
    let t5349 = t1680 * t583;
    let t5355 = t633 * t1778;
    let t5357 = t2735 * t198;
    (t5323, t5338, t5343, t5347, t5349, t5355, t5357)
}
