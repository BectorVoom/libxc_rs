//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 889/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk889(t2079: f64, t1572: f64, t4338: f64, t4345: f64, t5469: f64, t5562: f64, t6939: f64, t6942: f64, t6946: f64, t6958: f64, t6965: f64, t6971: f64, t6973: f64, t6977: f64, t6980: f64, t6983: f64) -> (f64, f64, f64) {
    let t7443 = t2079 * t2079;
    let t7444 = t7443 * t1572;
    let t7459 = -0.17648625e1_f64 * t6958 + 0.3529725e1_f64 * t6965 + t4338 + 0.34431666666666666666e0_f64 * t5469 - 0.34431666666666666667e0_f64 * t6939 + 0.103295e1_f64 * t6942 - 0.516475e0_f64 * t6946 + 0.31558125e0_f64 * t6971 + 0.6311625e0_f64 * t6973 + t4345 + 0.13892666666666666667e0_f64 * t5562 - 0.34731666666666666667e-1_f64 * t6977 + 0.20839e0_f64 * t6980 - 0.104195e0_f64 * t6983;
    (t7443, t7444, t7459)
}
