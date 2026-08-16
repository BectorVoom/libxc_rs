//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 868/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk868(t4035: f64, t7122: f64, t3833: f64, t5469: f64, t6939: f64, t6942: f64, t6946: f64, t6958: f64, t6965: f64, t1410: f64, t1897: f64, t3821: f64, t456: f64, t5510: f64, t6957: f64, t6964: f64) -> (f64, f64, f64) {
    let t7123 = t4035 * t7122;
    let t7138 = -0.991e-2_f64 * t6958 + 0.1982e-1_f64 * t6965 + t3833 + 0.27516666666666666666e-2_f64 * t5469 - 0.27516666666666666667e-2_f64 * t6939 + 0.8255e-2_f64 * t6942 - 0.41275e-2_f64 * t6946;
    let t7141 = -t3821 * t6957 / 8.0_f64 + t5510 * t1897 / 2.0_f64 + t1410 * t6964 / 4.0_f64 + t456 * t7138 / 2.0_f64;
    (t7123, t7138, t7141)
}
