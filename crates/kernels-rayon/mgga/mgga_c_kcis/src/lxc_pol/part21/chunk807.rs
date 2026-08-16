//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 807/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk807(t1004: f64, t110: f64, t285: f64, t2884: f64, t984: f64, t25: f64, t3041: f64, t3030: f64, t961: f64, t273: f64, t3033: f64, t2930: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9613 = t110 * t1004;
    let t9614 = t285 * t9613;
    let t9620 = t984 * t2884;
    let t9622 = t25 * t3041;
    let t9623 = t285 * t9622;
    let t9630 = 1.0_f64 / t3030 / t961;
    let t9634 = 1.0_f64 / t3033 / t273;
    let t9650 = t2930 * t930;
    (t9614, t9620, t9623, t9630, t9634, t9650)
}
