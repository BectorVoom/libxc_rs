//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1275/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1275(t100436: f64, t100501: f64, t100834: f64, t100841: f64, t100843: f64, t100847: f64, t100851: f64, t26955: f64, t26960: f64, t26966: f64, t29127: f64, t8087: f64, t92657: f64, t97015: f64) -> f64 {
    let t100862 = 0.30918233506944444445e-4_f64 * t100834 - 0.24734586805555555556e-3_f64 * t97015 * t8087 - 0.92673611111111111112e-3_f64 * t26966 * t29127 - 0.30952962962962962963e-2_f64 * t100841 - 0.25794135802469135802e-3_f64 * t100843 - 0.46336805555555555556e-3_f64 * t26960 * t100847 + 0.30891203703703703704e-3_f64 * t26960 * t100851 - 0.61836467013888888888e-4_f64 * t26955 * t100847 - 0.61890573922526041666e-5_f64 * t92657 * t100501 + 0.41224311342592592592e-4_f64 * t26955 * t100851 - 0.23168402777777777778e-3_f64 * t26960 * t100436;
    t100862
}
