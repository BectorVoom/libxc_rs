//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 767/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk767(t5710: f64, t5743: f64, t83: f64, t1882: f64, t7271: f64, t32065: f64, t32092: f64, t32061: f64, t32072: f64, t32080: f64, t32085: f64, t32089: f64, t32097: f64, t32101: f64, t32104: f64, t32109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32428 = t5710 * t5743;
    let t32429 = t83 * t32428;
    let t32433 = 2.0_f64 / 9.0_f64 * t1882 * t7271;
    let t32435 = 2.0_f64 / 3.0_f64 * t32065;
    let t32440 = t32092 / 3.0_f64;
    let t32445 = 3.0_f64 / 2.0_f64 * t32061 + t32435 + 2.0_f64 / 3.0_f64 * t32072 + 4.0_f64 * t32080 - 2.0_f64 * t32085 - t32089 / 2.0_f64 - t32440 - t32097 / 3.0_f64 - 3.0_f64 * t32101 + 2.0_f64 * t32104 + t32109 / 4.0_f64;
    (t32428, t32429, t32433, t32435, t32440, t32445)
}
