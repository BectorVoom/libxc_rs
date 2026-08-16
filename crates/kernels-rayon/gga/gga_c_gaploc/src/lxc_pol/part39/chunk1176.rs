//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1176/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1176(t2386: f64, t3689: f64, t544: f64, t6514: f64, t4130: f64, t2482: f64, t9272: f64, t12063: f64, t1424: f64, t2299: f64, t41670: f64, t41672: f64, t41675: f64, t41677: f64, t41681: f64, t41684: f64, t41687: f64, t41690: f64, t41692: f64) -> f64 {
    let t47846 = t544 * t6514 * t3689 * t2386;
    let t47848 = t4130 * t3689;
    let t47850 = t9272 * t47848 * t2482;
    let t47854 = t544 * t2299 * t12063 * t1424;
    let t47856 = -t41670 - 0.38342925953920749676e0_f64 * t41672 - t41675 - 0.25025342966295298669e1_f64 * t47846 - 0.57514388930881124514e0_f64 * t47850 - 0.39722766613167140743e-1_f64 * t47854 + t41677 + t41681 - t41684 - t41687 + t41690 - t41692;
    t47856
}
