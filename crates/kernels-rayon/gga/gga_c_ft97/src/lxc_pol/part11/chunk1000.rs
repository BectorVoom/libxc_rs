//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1000/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1000(t1882: f64, t9150: f64, t9268: f64, t9104: f64, t605: f64, t9132: f64, t9442: f64, t12703: f64, t144: f64, t1901: f64, t3439: f64, t379: f64, t39668: f64, t40522: f64, t40760: f64, t40766: f64, t40771: f64, t40772: f64, t40777: f64, t446: f64, t558: f64, t574: f64, t9304: f64, t9462: f64) -> f64 {
    let t40779 = t1882 * t9150;
    let t40784 = t1882 * t9268;
    let t40786 = t1882 * t9104;
    let t40792 = t9132 * t605;
    let t40800 = t1882 * t9442;
    let t40802 = -8.0_f64 / 9.0_f64 * t1901 * t3439 * t40766 * t40760 - 16.0_f64 / 9.0_f64 * t1901 * t3439 * t40771 * t40772 + 16.0_f64 / 27.0_f64 * t40777 + 4.0_f64 / 3.0_f64 * t40779 - 4.0_f64 / 3.0_f64 * t446 * t144 * t40522 + 4.0_f64 / 3.0_f64 * t40784 + 4.0_f64 / 3.0_f64 * t40786 - 4.0_f64 / 3.0_f64 * t446 * t574 * t9462 * t558 + 8.0_f64 / 3.0_f64 * t1901 * t40792 * t9304 * t379 - 8.0_f64 / 3.0_f64 * t1901 * t12703 * t39668 + 8.0_f64 / 3.0_f64 * t40800;
    t40802
}
