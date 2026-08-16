//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 117/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk117(t147: f64, t135: f64, t12: f64, t2: f64, t246: f64, t142: f64, t34: f64, t6: f64, t8: f64, t247: f64, t250: f64, t150: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t362 = t147 * t147;
    let t363 = 1.0_f64 / t362;
    let t364 = t135 * t363;
    let t367 = f64::sqrt(t12);
    let t368 = t367 * t2;
    let t369 = t368 * t246;
    let t374 = t142 * t6 / t34 / t8;
    let t376 = -0.632975e0_f64 * t247 - 0.29896666666666666667e0_f64 * t250 - 0.1023875e0_f64 * t369 - 0.82156666666666666667e-1_f64 * t374;
    let t377 = 1.0_f64 / t150;
    (t362, t363, t364, t368, t369, t374, t376, t377)
}
