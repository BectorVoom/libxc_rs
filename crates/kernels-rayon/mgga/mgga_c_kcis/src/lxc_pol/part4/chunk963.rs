//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 963/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk963(t9588: f64, t3206: f64, t9429: f64, t2867: f64, t987: f64, t25: f64, t2912: f64, t285: f64, t1004: f64, t110: f64, t2884: f64, t984: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9589 = t9588 * sigma0;
    let t9600 = t9429 * t3206;
    let t9608 = t2867 * t987;
    let t9610 = t25 * t2912;
    let t9611 = t285 * t9610;
    let t9613 = t110 * t1004;
    let t9614 = t285 * t9613;
    let t9620 = t984 * t2884;
    (t9589, t9600, t9608, t9611, t9614, t9620)
}
