//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1113/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1113(t27048: f64, t27101: f64, t27176: f64, t305: f64, t321: f64, t326: f64, t352: f64, t36013: f64, t37584: f64, t41463: f64, t42637: f64, t43065: f64, t43658: f64, t43692: f64, t44183: f64, t44194: f64, t5148: f64, t5266: f64, t793: f64, t794: f64, t839: f64, t876: f64, t9523: f64, t9551: f64) -> f64 {
    let t44230 = -0.5987120850931904282e-1_f64 * t41463 - 0.23948483403727617128e0_f64 * t5148 * t44183 * t321 + 0.71845450211182851384e0_f64 * t27048 * t42637 + 0.23948483403727617128e0_f64 * t36013 + t37584 + 0.59871208509319042821e-1_f64 * t305 * t43658 - 0.59871208509319042821e-1_f64 * t326 * t43065 - 0.23948483403727617128e0_f64 * t27101 * t9551 * t794 - 0.47896966807455234256e0_f64 * t27176 * t9551 * t839 + 0.23948483403727617128e0_f64 * t5266 * t44194 * t352 + 0.11974241701863808564e0_f64 * t793 * t43692 + 0.35922725105591425692e0_f64 * t27048 * t9523 * t876;
    t44230
}
