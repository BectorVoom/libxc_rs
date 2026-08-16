//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1149/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1149(t1035: f64, t18857: f64, t1072: f64, t6307: f64, t331: f64, t6313: f64, t1027: f64, t6317: f64, t6353: f64, t829: f64, t6272: f64, t1045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19327 = t1035 * t18857;
    let t19330 = t1072 * t6307;
    let t19332 = t331 * t6313;
    let t19334 = t1027 * t6317;
    let t19336 = t1027 * t6353;
    let t19340 = t6307 * t829;
    let t19343 = t1035 * t6272;
    let t19344 = t19343 * t1045;
    (t19327, t19330, t19332, t19334, t19336, t19340, t19344)
}
