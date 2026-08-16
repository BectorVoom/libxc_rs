//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1074/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1074(t2173: f64, t26717: f64, t3220: f64, t356: f64, t303: f64, t26673: f64, t26677: f64, t26681: f64, t26685: f64, t26688: f64, t26692: f64, t26697: f64, t26703: f64, t26708: f64, t26712: f64, t26715: f64, t7687: f64, t7703: f64, t7706: f64, t7711: f64) -> (f64, f64, f64, f64) {
    let t26718 = t2173 * t26717;
    let t26720 = t356 * t3220;
    let t26721 = t303 * t26720;
    let t26725 = -0.88437037037037037034e-2_f64 * t26673 - 0.33163888888888888888e-2_f64 * t26677 + 0.46336805555555555556e-3_f64 * t7703 * t26681 - 0.18550940104166666667e-3_f64 * t26685 * t26688 + 0.12356481481481481482e-2_f64 * t26692 * t7706 - 0.30891203703703703704e-3_f64 * t7703 * t26697 - 0.13901041666666666667e-2_f64 * t7703 * t26688 + 0.61836467013888888889e-4_f64 * t26685 * t26703 + 0.16581944444444444444e-2_f64 * t26708 + 0.27636574074074074073e-2_f64 * t26712 + 0.46336805555555555556e-3_f64 * t26715 + 0.46336805555555555556e-3_f64 * t26718 - 0.55273148148148148147e-3_f64 * t26721 + 0.13901041666666666667e-2_f64 * t7687 * t7711;
    (t26718, t26720, t26721, t26725)
}
