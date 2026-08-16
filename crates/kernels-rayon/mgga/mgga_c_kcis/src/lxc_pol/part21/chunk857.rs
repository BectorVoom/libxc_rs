//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 857/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk857(t13207: f64, t4994: f64, t1023: f64, t13181: f64, t1020: f64, t2830: f64, t4999: f64, t1646: f64, t3190: f64, t3211: f64, t3210: f64, t3200: f64) -> (f64, f64, f64, f64, f64) {
    let t13208 = t4994 * t13207;
    let t13210 = t13181 * t1023;
    let t13211 = t1020 * t13210;
    let t13213 = t4999 * t2830;
    let t13214 = t1020 * t13213;
    let t13217 = t3211 * t1646 * t3190;
    let t13218 = t3210 * t13217;
    let t13219 = t3200 * t13218;
    (t13208, t13211, t13214, t13217, t13219)
}
