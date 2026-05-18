//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 670/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk670<F: Float>(t5346: F, t561: F, t1680: F, t583: F, t1781: F, t631: F, t184: F, t221: F, t1778: F, t633: F, t198: F, t2735: F) -> (F, F, F, F, F, F, F) {
    let t5347 = t561 * t5346;
    let t5348 = F::new(8.0) / F::new(15.0) * t5347;
    let t5349 = t1680 * t583;
    let t5350 = F::new(8.0) / F::new(15.0) * t5349;
    let t5351 = t1781 * t631;
    let t5352 = t5351 * t184;
    let t5354 = F::new(4.0) / F::new(5.0) * t5352 * t221;
    let t5355 = t633 * t1778;
    let t5356 = F::new(4.0) / F::new(45.0) * t5355;
    let t5357 = t2735 * t198;
    (t5348, t5350, t5351, t5352, t5354, t5356, t5357)
}
