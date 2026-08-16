//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1260/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1260(t27594: f64, t6140: f64, t12825: f64, t7978: f64, t8221: f64, t27591: f64, t28727: f64, t28714: f64, t98225: f64, t54162: f64, t8212: f64, t98254: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t99133 = t27594 * t6140;
    let t99152 = t7978 * t12825 * t8221;
    let t99154 = t28727 * t27591;
    let t99157 = 0.7722800925925925926e-4_f64 * t28714 * t27591;
    let t99173 = 0.10317654320987654321e-2_f64 * t98225;
    let t99175 = t54162 * t8212;
    let t99176 = t7978 * t99175;
    let t99193 = 0.25794135802469135802e-2_f64 * t98254;
    (t99133, t99152, t99154, t99157, t99173, t99175, t99176, t99193)
}
