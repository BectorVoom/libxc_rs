//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1153/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1153(t104: f64, t9447: f64, t1427: f64, t1954: f64, t2166: f64, t24589: f64, t32262: f64, t32283: f64, t32298: f64, t32301: f64, t36611: f64, t36729: f64, t36744: f64, t36747: f64, t36750: f64, t36753: f64, t36755: f64, t567: f64, t7297: f64, t8040: f64, t8372: f64, t9096: f64, t9448: f64) -> f64 {
    let t36756 = t104 * t9447;
    let t36760 = 12.0_f64 * t1427 * t32262 * t8372 + 6.0_f64 * t1954 * t36756 * t567 - 2.0_f64 * t2166 * t567 * t9448 - 6.0_f64 * t24589 * t7297 * t8040 - 6.0_f64 * t36611 * t36729 * t9096 - 2.0_f64 * t32283 + 3.0_f64 * t32298 + 6.0_f64 * t32301 - t36744 + t36747 - t36750 + t36753 + t36755;
    t36760
}
