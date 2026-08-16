//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 860/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk860(t1356: f64, t3918: f64, t7002: f64, t3926: f64, t3933: f64, t5469: f64, t5562: f64, t6939: f64, t6942: f64, t6946: f64, t6958: f64, t6965: f64, t6971: f64, t6973: f64, t6977: f64, t6980: f64, t6983: f64) -> (f64, f64) {
    let t7004 = t3918 * t7002 * t1356;
    let t7019 = -0.1294625e1_f64 * t6958 + 0.258925e1_f64 * t6965 + t3926 + 0.20128333333333333334e0_f64 * t5469 - 0.20128333333333333333e0_f64 * t6939 + 0.60385e0_f64 * t6942 - 0.301925e0_f64 * t6946 + 0.82524375e-1_f64 * t6971 + 0.16504875e0_f64 * t6973 + t3933 + 0.11038e0_f64 * t5562 - 0.27595e-1_f64 * t6977 + 0.16557e0_f64 * t6980 - 0.82785e-1_f64 * t6983;
    (t7004, t7019)
}
