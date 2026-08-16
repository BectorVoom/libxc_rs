//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1061/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1061(t3212: f64, t4566: f64, t13410: f64, t4554: f64, t4823: f64, t922: f64, t9517: f64, t3200: f64, t4807: f64, t9425: f64, t4549: f64, t1085: f64, t2840: f64) -> (f64, f64, f64, f64, f64) {
    let t13416 = t4566 * t3212;
    let t13417 = t13410 * t13416;
    let t13418 = t4554 * t13417;
    let t13420 = t4823 * t922;
    let t13421 = t9517 * t13420;
    let t13422 = t3200 * t13421;
    let t13424 = t9425 * t4807;
    let t13426 = t9425 * t4549;
    let t13428 = t2840 * t1085;
    (t13418, t13422, t13424, t13426, t13428)
}
