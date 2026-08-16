//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 646/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk646(t671: f64, t6990: f64, t6992: f64, t8616: f64, t8868: f64, t8872: f64, t8876: f64, t8880: f64, t8884: f64, t8887: f64, t8890: f64, t8942: f64, t8949: f64, t8952: f64, t8956: f64, t8960: f64) -> f64 {
    let t8963 = -0.88437037037037037034e-2_f64 * t8868 - 0.33163888888888888888e-2_f64 * t8872 - 0.55273148148148148147e-3_f64 * t8876 - 0.88437037037037037034e-2_f64 * t8880 + 0.16581944444444444444e-2_f64 * t8884 - 0.49745833333333333332e-2_f64 * t8887 + 0.33163888888888888888e-2_f64 * t8890 - 0.24872916666666666666e-2_f64 * t8942 - 0.88437037037037037034e-2_f64 * t6990 + 0.33163888888888888888e-2_f64 * t6992 - 0.33163888888888888888e-2_f64 * t8949 + 0.33163888888888888888e-2_f64 * t8952 + 0.16581944444444444444e-2_f64 * t8956 + 0.27636574074074074073e-2_f64 * t8960 + t8616 * t671;
    t8963
}
