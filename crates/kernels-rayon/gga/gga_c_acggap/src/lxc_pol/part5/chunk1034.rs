//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1034/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1034(t1089: f64, t1421: f64, t384: f64, t966: f64, t13087: f64, t4908: f64, t13263: f64, t1562: f64, t3379: f64, t4701: f64, t361: f64, t435: f64) -> (f64, f64, f64, f64, f64) {
    let t17729 = t384 * t1089 * t966 * t1421;
    let t17733 = t13087 * t4908;
    let t17740 = t13263 * t1562;
    let t17742 = t3379 * t4701;
    let t17752 = t361 * t435;
    (t17729, t17733, t17740, t17742, t17752)
}
