//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1328/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1328(t14725: f64, t14726: f64, t2971: f64, t2984: f64, t5026: f64, t5032: f64, t5036: f64, t5405: f64, t5409: f64, t6020: f64, t6594: f64, t6598: f64, t6601: f64) -> f64 {
    let t24683 = 12.0_f64 * t5405 + 24.0_f64 * t5409 + 0.70178683471615754484e1_f64 * t5026 + 6.0_f64 * t6594 + 192.0_f64 * t2971 - t14725 - t14726 - 0.35089341735807877242e1_f64 * t2984 + 120.0_f64 * t5032 - t6020 - 64.0_f64 * t5036 + 4.0_f64 * t6598 + 24.0_f64 * t6601;
    t24683
}
