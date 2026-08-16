//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1081/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1081(t11529: f64, t11534: f64, t11536: f64, t2841: f64, t2843: f64, t2845: f64, t2894: f64, t4039: f64, t4048: f64, t4052: f64, t5508: f64, t6579: f64) -> f64 {
    let t19364 = t11529 + 6.0_f64 * t5508 + 2.0_f64 * t6579 + 16.0_f64 * t4039 - 48.0_f64 * t2841 - 8.0_f64 * t2843 - 8.0_f64 * t2845 + t11534 + t11536 - 0.14649157844805236044e-2_f64 * t4048 - 48.0_f64 * t2894 + 12.0_f64 * t4052;
    t19364
}
