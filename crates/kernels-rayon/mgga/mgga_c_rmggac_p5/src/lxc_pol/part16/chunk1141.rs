//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1141/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1141(t6491: f64, t702: f64, t289: f64, t36804: f64, t36809: f64, t38079: f64, t38080: f64, t43937: f64, t43948: f64, t47721: f64, t47723: f64, t47725: f64, t47727: f64, t47729: f64, t47735: f64, t47737: f64, t47740: f64, t47743: f64, t47745: f64, t47747: f64) -> f64 {
    let t49655 = t6491 * t702;
    let t49666 = 0.5454932330849068346e-1_f64 * t47721 + 0.5454932330849068346e-1_f64 * t47723 + 0.40911992481368012595e-1_f64 * t47725 - 0.5454932330849068346e-1_f64 * t47727 - 0.40911992481368012595e-1_f64 * t47729 - 0.2363e1_f64 * t289 * t49655 - t38079 + t38080 + 0.162600798888400151e-2_f64 * t36804 + 0.162600798888400151e-2_f64 * t36809 - t43937 - 0.71845450211182851384e0_f64 * t47735 - 0.17961362552795712846e0_f64 * t47737 - 0.17961362552795712846e0_f64 * t47740 - 0.17961362552795712846e0_f64 * t47743 + t43948 - 0.40911992481368012596e-1_f64 * t47745 - 0.40911992481368012596e-1_f64 * t47747;
    t49666
}
