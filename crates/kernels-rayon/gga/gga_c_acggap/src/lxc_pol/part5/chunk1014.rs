//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1014/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1014(t12727: f64, t1558: f64, t13298: f64, t13364: f64, t1421: f64, t3169: f64, t13299: f64, t3176: f64, t13285: f64, t3073: f64, t1101: f64, t176: f64, t406: f64, t8790: f64) -> (f64, f64, f64, f64, f64) {
    let t17148 = t12727 * t1558;
    let t17152 = t13298 * t13364 * t1421 * t3169;
    let t17156 = t13298 * t13299 * t1421 * t3176;
    let t17167 = t3073 * t13285;
    let t17171 = t17167 * t176 * t8790 * t1101 * t406;
    (t17148, t17152, t17156, t17167, t17171)
}
