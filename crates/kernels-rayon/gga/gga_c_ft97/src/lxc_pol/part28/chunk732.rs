//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 732/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk732(t32106: f64, t469: f64, t28: f64, t5665: f64, t32061: f64, t32066: f64, t32072: f64, t32080: f64, t32085: f64, t32089: f64, t32093: f64, t32097: f64, t32101: f64, t32104: f64) -> (f64, f64, f64) {
    let t32107 = t469 * t32106;
    let t32109 = t5665 * t28 * t32107;
    let t32111 = t32061 / 2.0_f64 + t32066 + 2.0_f64 / 9.0_f64 * t32072 + 4.0_f64 / 3.0_f64 * t32080 - 2.0_f64 / 3.0_f64 * t32085 - t32089 / 6.0_f64 - t32093 - t32097 / 9.0_f64 - t32101 + 2.0_f64 / 3.0_f64 * t32104 + t32109 / 12.0_f64;
    (t32107, t32109, t32111)
}
