//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 962/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk962(t7799: f64, t8545: f64, t30260: f64, t8491: f64, t30402: f64, t31309: f64, t525: f64, t7325: f64, t30273: f64, t30280: f64, t31362: f64, t8783: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34056 = t7799 * t8545;
    let t34058 = 0.13976929906490734252e-1_f64 * t30260;
    let t34059 = t7799 * t8491;
    let t34068 = t31309 * t30402 * t7325 * t525;
    let t34076 = 0.21437009059034868486e-3_f64 * t30273;
    let t34077 = 0.28582678745379824648e-3_f64 * t30280;
    let t34081 = t31362 * t8783;
    (t34056, t34058, t34059, t34068, t34076, t34077, t34081)
}
