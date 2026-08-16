//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1380/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1380(t103662: f64, t27339: f64, t102655: f64, t102658: f64, t102661: f64, t102666: f64, t102669: f64, t103069: f64, t28388: f64, t98830: f64, t98845: f64, t98849: f64, t98854: f64, t98864: f64) -> f64 {
    let t103702 = t27339 * t103662;
    let t103712 = 0.88437037037037037033e-2_f64 * t102655 - 0.61890573922526041667e-5_f64 * t103702 + 0.13265555555555555555e-1_f64 * t102658 - 0.88437037037037037033e-2_f64 * t102661 - 0.7369753086419753086e-3_f64 * t98830 - 0.37134344353515625e-4_f64 * t28388 * t103069 + 0.1621345679012345679e-1_f64 * t102666 - 0.92673611111111111112e-3_f64 * t98845 - t98849 - t98854 + 0.16581944444444444444e-2_f64 * t102669 - t98864;
    t103712
}
