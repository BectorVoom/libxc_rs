//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1279/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1279(t1020: f64, t26753: f64, t4796: f64, t14640: f64, t7718: f64, t27873: f64, t9386: f64, t1092: f64, t3219: f64, t42570: f64, t27796: f64, t2822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95566 = t1020 * t26753 * t4796;
    let t95569 = t1020 * t7718 * t14640;
    let t95571 = t9386 * t27873;
    let t95572 = 0.3684876543209876543e-2_f64 * t95571;
    let t95579 = t1092 * t7718 * t42570 * t3219;
    let t95581 = t2822 * t27796;
    (t95566, t95569, t95571, t95572, t95579, t95581)
}
