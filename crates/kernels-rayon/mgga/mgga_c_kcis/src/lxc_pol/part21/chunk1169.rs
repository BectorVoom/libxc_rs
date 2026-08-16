//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1169/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1169(t1291: f64, t8108: f64, t1281: f64, t8104: f64, t28012: f64, t28014: f64, t28016: f64, t28018: f64, t28020: f64, t28022: f64, t28025: f64, t28027: f64, t28030: f64, t28032: f64, t28034: f64, t28036: f64, t28038: f64) -> (f64, f64, f64) {
    let t28260 = t8108 * t1291;
    let t28265 = t8104 * t1281;
    let t28280 = -0.25e0_f64 * t28012 + 0.9375e-1_f64 * t28014 - 0.20234375e-1_f64 * t28016 + 0.625e-1_f64 * t28018 - 0.10791666666666666667e0_f64 * t28020 + 0.14388888888888888889e0_f64 * t28022 - 0.89930555555555555557e-2_f64 * t28025 + 0.20234375e-1_f64 * t28027 - 0.4046875e-1_f64 * t28030 - 0.20833333333333333333e-1_f64 * t28032 + 0.26979166666666666667e-1_f64 * t28034 - 0.625e-1_f64 * t28036 - 0.26979166666666666667e-1_f64 * t28038;
    (t28260, t28265, t28280)
}
