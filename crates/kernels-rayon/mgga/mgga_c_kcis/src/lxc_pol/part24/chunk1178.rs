//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1178/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1178(t1000: f64, t1245: f64, t1009: f64, t9494: f64, t44575: f64, t7703: f64, t7705: f64, t9372: f64, t1071: f64, t2811: f64, t2836: f64, t982: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93435 = t1245 * t1000;
    let t93463 = t1009 * t9494;
    let t93471 = t7703 * t44575 * t7705;
    let t93485 = t1009 * t9372;
    let t93508 = t2811 * t1071;
    let t93562 = t2836 * t982 * t990;
    (t93435, t93463, t93471, t93485, t93508, t93562)
}
