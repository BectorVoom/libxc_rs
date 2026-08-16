//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 427/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk427(t209: f64, t2746: f64, t880: f64, t208: f64, t214: f64, t2733: f64, t2742: f64, t876: f64, t884: f64) -> (f64, f64) {
    let t2748 = t209 * t880 * t2746;
    let t2751 = 35.0_f64 / 432.0_f64 * t2733 * t214 + 7.0_f64 / 144.0_f64 * t876 * t884 + t208 * t2742 / 48.0_f64 - t208 * t2748 / 96.0_f64;
    (t2748, t2751)
}
