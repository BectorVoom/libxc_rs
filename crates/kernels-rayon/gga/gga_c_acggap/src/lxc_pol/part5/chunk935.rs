//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 935/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk935(t3055: f64, t3058: f64, t1261: f64, t848: f64, t1035: f64, t12254: f64, t452: f64, t3828: f64, t864: f64, t3088: f64, t407: f64, t441: f64) -> (f64, f64, f64, f64, f64) {
    let t14491 = t3055 * t3058;
    let t14495 = t848 * t1261;
    let t14501 = 0.52683593463484092788e1_f64 * t1035 * t452 * t12254;
    let t14503 = t1035 * t3828 * t864;
    let t14518 = t3088 * t441 * t864 * t407;
    (t14491, t14495, t14501, t14503, t14518)
}
