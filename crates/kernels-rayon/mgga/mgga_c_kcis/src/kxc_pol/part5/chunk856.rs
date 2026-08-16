//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 856/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk856(t3820: f64, t6957: f64, t3794: f64, t5469: f64, t6939: f64, t6942: f64, t6946: f64) -> (f64, f64) {
    let t6958 = t3820 * t6957;
    let t6964 = t3794 + 2.0_f64 / 9.0_f64 * t5469 - 2.0_f64 / 9.0_f64 * t6939 + 2.0_f64 / 3.0_f64 * t6942 - t6946 / 3.0_f64;
    (t6958, t6964)
}
