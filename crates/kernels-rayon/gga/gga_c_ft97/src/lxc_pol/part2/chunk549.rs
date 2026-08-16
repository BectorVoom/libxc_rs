//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 549/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk549(t2092: f64, t2093: f64, t2095: f64, t3139: f64, t3497: f64, t3500: f64, t3503: f64, t3507: f64, t3510: f64, t3513: f64, t3515: f64, t3520: f64, t3524: f64, t462: f64, t92: f64) -> f64 {
    let t3526 = t2092 + t2093 / 9.0_f64 + t2095 / 3.0_f64 + t3497 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t462 * t3500 + t462 * t3503 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t462 * t3507 - 2.0_f64 / 3.0_f64 * t3139 * t3510 + t3513 / 3.0_f64 + t462 * t3515 / 3.0_f64 + 2.0_f64 * t462 * t3520 - t92 * t3524;
    t3526
}
