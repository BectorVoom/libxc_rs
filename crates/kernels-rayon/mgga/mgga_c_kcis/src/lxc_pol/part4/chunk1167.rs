//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1167/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1167(t14788: f64, t5076: f64, t1184: f64, t5086: f64, t1165: f64, t284: f64, t5078: f64, t14766: f64, t14769: f64, t14771: f64, t14773: f64, t14776: f64, t14779: f64, t14783: f64, t14786: f64) -> (f64, f64, f64, f64) {
    let t14789 = t5076 * t14788;
    let t14791 = t1184 * t5086;
    let t14793 = t1165 * t284;
    let t14794 = t14793 * t5078;
    let t14796 = -t14766 / 64.0_f64 + t14769 / 72.0_f64 - t14771 / 12.0_f64 - 2.0_f64 / 9.0_f64 * t14773 - 19.0_f64 / 108.0_f64 * t14776 - t14779 / 24.0_f64 + t14783 / 8.0_f64 + t14786 / 96.0_f64 - t14789 / 72.0_f64 + t14791 / 18.0_f64 - t14794 / 36.0_f64;
    (t14789, t14791, t14794, t14796)
}
