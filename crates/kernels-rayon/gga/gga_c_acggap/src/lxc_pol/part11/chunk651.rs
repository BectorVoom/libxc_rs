//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 651/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk651(t1165: f64, t3176: f64, t4267: f64, t1017: f64, t960: f64, t1322: f64, t922: f64, t1315: f64, t3621: f64, t4417: f64, t1137: f64, t1319: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5157 = t1165 * t4267 * t3176;
    let t5160 = t4267 * t1017;
    let t5161 = t960 * t5160;
    let t5164 = t1322 * t922;
    let t5165 = t960 * t5164;
    let t5169 = 7.0_f64 / 24.0_f64 * t3621 * t1315;
    let t5170 = t4417 * t1017;
    let t5171 = t960 * t5170;
    let t5175 = 7.0_f64 / 72.0_f64 * t1137 * t1319;
    (t5157, t5160, t5161, t5164, t5165, t5169, t5170, t5171, t5175)
}
