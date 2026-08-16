//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1049/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1049(t149920: f64, t193: f64, t713: f64, t89: f64, t141340: f64, t141364: f64, t141368: f64, t150953: f64, t150958: f64, t150962: f64, t150966: f64, t150971: f64, t150974: f64, t150977: f64, t150980: f64, t150983: f64, t150985: f64, t150988: f64, t150992: f64) -> (f64, f64) {
    let t150996 = t89 * t193 * t149920 * t713;
    let t150998 = -2.0_f64 / 3.0_f64 * t141340 + 3.0_f64 * t150953 - 15.0_f64 / 4.0_f64 * t150958 - t150962 / 2.0_f64 + 4.0_f64 / 3.0_f64 * t150966 + t150971 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t150974 - 4.0_f64 / 9.0_f64 * t150977 + 2.0_f64 * t150980 - 4.0_f64 / 3.0_f64 * t150983 + t141364 - t141368 + 2.0_f64 / 9.0_f64 * t150985 + t150988 / 6.0_f64 + 4.0_f64 * t150992 + 2.0_f64 * t150996;
    (t150996, t150998)
}
