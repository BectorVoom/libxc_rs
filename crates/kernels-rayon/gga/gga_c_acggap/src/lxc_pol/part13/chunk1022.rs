//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1022/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1022(t30371: f64, t5152: f64, t5138: f64, t8511: f64, t30273: f64, t30280: f64, t5143: f64, t31362: f64, t8783: f64, t1165: f64, t20595: f64, t604: f64, t7337: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34072 = t30371 * t5152;
    let t34074 = t8511 * t5138;
    let t34076 = 0.21437009059034868486e-3_f64 * t30273;
    let t34077 = 0.28582678745379824648e-3_f64 * t30280;
    let t34078 = t8511 * t5143;
    let t34081 = t31362 * t8783;
    let t34082 = 0.15724046144802076034e-2_f64 * t34081;
    let t34085 = t7337 * t1165 * t604 * t20595;
    (t34072, t34074, t34076, t34077, t34078, t34082, t34085)
}
