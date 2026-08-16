//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1292/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1292(t98225: f64, t54162: f64, t8212: f64, t7978: f64, t1615: f64, t167: f64, t18079: f64, t28752: f64, t98254: f64, t1370: f64, t7969: f64, t833: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99173 = 0.10317654320987654321e-2_f64 * t98225;
    let t99175 = t54162 * t8212;
    let t99176 = t7978 * t99175;
    let t99184 = t18079 * t28752 * t167 * t1615;
    let t99193 = 0.25794135802469135802e-2_f64 * t98254;
    let t99198 = t1370 * t7969;
    let t99199 = t833 * t1615;
    (t99173, t99175, t99176, t99184, t99193, t99198, t99199)
}
