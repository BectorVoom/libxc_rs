//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1134/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1134(t10611: f64, t458: f64, t10572: f64, t2: f64, t33828: f64, t1771: f64, t2787: f64, t10563: f64, t1775: f64, t798: f64, t9567: f64, t10613: f64, t14961: f64, t192: f64, t2771: f64, t41454: f64, t41464: f64, t41473: f64, t41490: f64, t4199: f64, t4206: f64, t42145: f64, t43359: f64, t43420: f64, t43428: f64, t43469: f64, t43525: f64, t462: f64, t848: f64, t92: f64) -> (f64, f64) {
    let t43799 = t458 * t10611;
    let t43801 = t458 * t10572;
    let t43803 = t33828 * t2;
    let t43808 = t1771 * t2787;
    let t43831 = t1775 * t10563;
    let t43833 = t9567 * t798;
    let t43834 = t43833 * t2;
    let t43841 = 4.0_f64 / 3.0_f64 * t43799 + 8.0_f64 * t43801 + 24.0_f64 * t92 * t192 * t43803 * t43525 - 8.0_f64 / 3.0_f64 * t43808 - 16.0_f64 / 3.0_f64 * t462 * t10613 * t43359 + 8.0_f64 * t462 * t4199 * t41464 - 20.0_f64 / 9.0_f64 * t462 * t14961 * t41454 + 4.0_f64 / 3.0_f64 * t462 * t2771 * t43428 + 8.0_f64 / 3.0_f64 * t462 * t4206 * t41490 - 8.0_f64 / 9.0_f64 * t462 * t4199 * t41473 + 4.0_f64 / 3.0_f64 * t462 * t10613 * t43420 + 8.0_f64 / 9.0_f64 * t43831 + 40.0_f64 / 27.0_f64 * t462 * t43834 * t43469 - t462 * t848 * t42145 / 3.0_f64;
    (t43833, t43841)
}
