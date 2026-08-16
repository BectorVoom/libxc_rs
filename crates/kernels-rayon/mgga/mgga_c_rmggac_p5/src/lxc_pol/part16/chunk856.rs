//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 856/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk856(t8595: f64, t8598: f64, t8604: f64, t8610: f64, t8612: f64, t8617: f64, t9371: f64, t8623: f64, t8627: f64, t8633: f64, t8637: f64, t8643: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42465 = 0.1702583995731913576e-4_f64 * t8595;
    let t42466 = 0.1702583995731913576e-4_f64 * t8598;
    let t42468 = 0.1702583995731913576e-4_f64 * t8604;
    let t42469 = 0.1702583995731913576e-4_f64 * t8610;
    let t42470 = 0.212822999466489197e-4_f64 * t8612;
    let t42471 = 0.212822999466489197e-4_f64 * t8617;
    let t42472 = 0.11974241701863808564e0_f64 * t9371;
    let t42473 = 0.2727466165424534173e-1_f64 * t8623;
    let t42474 = 0.16364796992547205038e0_f64 * t8627;
    let t42475 = 0.2727466165424534173e0_f64 * t8633;
    let t42476 = 0.5454932330849068346e-1_f64 * t8637;
    let t42477 = 0.81823984962736025192e-1_f64 * t8643;
    (t42465, t42466, t42468, t42469, t42470, t42471, t42472, t42473, t42474, t42475, t42476, t42477)
}
