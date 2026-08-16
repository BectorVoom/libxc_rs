//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1043/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1043(t167: f64, t829: f64, t4546: f64, t3210: f64, t13172: f64, t3183: f64, t4999: f64, t1092: f64, t5168: f64, t1134: f64, t1800: f64, t2850: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13173 = t167 * t829;
    let t13174 = t4546 * t13173;
    let t13175 = t3210 * t13174;
    let t13176 = t13172 * t13175;
    let t13178 = t4999 * t3183;
    let t13179 = t1092 * t13178;
    let t13181 = t5168 * sigma0;
    let t13182 = t13181 * t1134;
    let t13183 = t1092 * t13182;
    let t13186 = t1800 * t2850;
    (t13173, t13176, t13179, t13181, t13183, t13186)
}
