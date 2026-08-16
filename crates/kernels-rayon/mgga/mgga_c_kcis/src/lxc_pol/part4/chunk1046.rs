//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1046/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1046(t2830: f64, t4999: f64, t1020: f64, t1646: f64, t3190: f64, t3211: f64, t3210: f64, t3200: f64, t4806: f64, t9438: f64, t4548: f64, t4556: f64) -> (f64, f64, f64, f64, f64) {
    let t13213 = t4999 * t2830;
    let t13214 = t1020 * t13213;
    let t13217 = t3211 * t1646 * t3190;
    let t13218 = t3210 * t13217;
    let t13219 = t3200 * t13218;
    let t13221 = t9438 * t4806;
    let t13222 = t3200 * t13221;
    let t13224 = t9438 * t4548;
    let t13225 = t3200 * t13224;
    let t13227 = t9438 * t4556;
    (t13214, t13219, t13222, t13225, t13227)
}
