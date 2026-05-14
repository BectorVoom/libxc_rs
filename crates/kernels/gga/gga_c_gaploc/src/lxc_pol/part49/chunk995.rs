//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 995/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk995<F: Float>(t3689: F, t4130: F, t2482: F, t9272: F, t12063: F, t1424: F, t2299: F, t544: F, t41670: F, t41672: F, t41675: F, t41677: F, t41681: F, t41684: F, t41687: F, t41690: F, t41692: F, t47846: F) -> (F,) {
    let t47848 = t4130 * t3689;
    let t47850 = t9272 * t47848 * t2482;
    let t47854 = t544 * t2299 * t12063 * t1424;
    let t47856 = -t41670 - 0.38342925953920749676e0 * t41672 - t41675 - 0.25025342966295298669e1 * t47846 - 0.57514388930881124514e0 * t47850 - 0.39722766613167140743e-1 * t47854 + t41677 + t41681 - t41684 - t41687 + t41690 - t41692;
    (t47856,)
}
