//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 742/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk742<F: Float>(t2331: F, t899: F, t900: F, t935: F, t2323: F, t2327: F, t2206: F, t2212: F, t4379: F, t904: F, t933: F, t6466: F, t6474: F, t6477: F, t6482: F, t6486: F, t6490: F, t6495: F, t6497: F, t6502: F, t902: F, t914: F, t929: F) -> (F, F, F, F) {
    let t6505 = t899 * t900 * t2331;
    let t6506 = t6505 * t935;
    let t6508 = t2323 * t2327;
    let t6510 = t2206 * t2212;
    let t6511 = 7.0 / 16.0 * t6510;
    let t6513 = t933 * t904 * t4379;
    let t6516 = t902 * t6466 / 1536.0 + t902 * t6474 / 384.0 - 7.0 / 384.0 * t6477 - t6482 - t6486 + t6490 + t6495 - t914 * t6497 / 1536.0 - 7.0 / 256.0 * t6502 - 119.0 / 1152.0 * t6506 + 7.0 / 384.0 * t6508 - t6511 - t929 * t6513 / 768.0;
    (t6505, t6511, t6513, t6516)
}
