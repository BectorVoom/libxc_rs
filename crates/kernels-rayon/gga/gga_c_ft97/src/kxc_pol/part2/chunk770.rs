//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 770/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk770(t10952: f64, t10962: f64, t10965: f64, t10967: f64, t10970: f64, t12067: f64, t1538: f64, t1761: f64, t1920: f64, t3109: f64, t3289: f64, t438: f64, t497: f64, t948: f64, t984: f64) -> f64 {
    let t12068 = -t1538 * t984 - t1761 * t984 - t1920 * t948 - 2.0_f64 * t3109 * t497 - 2.0_f64 * t3289 * t438 - 12.0_f64 * t10952 + 8.0_f64 * t10962 + 4.0_f64 * t10965 + 8.0_f64 * t10967 + 4.0_f64 * t10970 + t12067;
    t12068
}
