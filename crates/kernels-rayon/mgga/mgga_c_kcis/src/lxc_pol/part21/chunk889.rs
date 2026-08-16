//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 889/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk889(t1072: f64, t4833: f64, t331: f64, t4837: f64, t1717: f64, t2635: f64, t4840: f64, t829: f64, t3096: f64, t4836: f64, t1035: f64, t167: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13665 = 0.93706135855523581992e-2_f64 * t1072 * t4833;
    let t13667 = 0.93706135855523581992e-2_f64 * t331 * t4837;
    let t13668 = t1717 * t2635;
    let t13671 = t4840 * t829;
    let t13674 = t4836 * t3096;
    let t13677 = t1035 * t167;
    (t13665, t13667, t13668, t13671, t13674, t13677)
}
