//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 811/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk811(t13587: f64, t7: f64, t118: f64, t2474: f64, t5: f64, t22: f64, t4864: f64, t41: f64, t85: f64, t8565: f64, t11: f64, t119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    let t13716 = t5 * t2474;
    let t13948 = t22 * t4864;
    let t14249 = t85 * t8565 * t41;
    let t14954 = t11 * t41;
    let t14955 = t85 * t14954;
    let t15007 = t119 * t41;
    (t13589, t13716, t13948, t14249, t14955, t15007)
}
