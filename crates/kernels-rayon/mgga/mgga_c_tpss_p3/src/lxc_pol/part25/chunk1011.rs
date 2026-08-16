//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1011/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1011(t13614: f64, t13625: f64, t13954: f64, t13972: f64, t1165: f64, t13133: f64, t1338: f64, t13452: f64, t13458: f64, t13546: f64, t13554: f64, t13565: f64, t2056: f64, t3493: f64, t3537: f64, t4347: f64, t4674: f64, t6234: f64, t645: f64) -> (f64, f64) {
    let t13974 = t13614 + t13625 + t13954 + t13972;
    let t14001 = 2.0_f64 * t1165 * t13546 + 4.0_f64 * t13133 * t1338 + 4.0_f64 * t1338 * t13554 + 2.0_f64 * t13565 * t645 + 2.0_f64 * t2056 * t4674 + 4.0_f64 * t3493 * t3537 + 4.0_f64 * t3537 * t6234 + 2.0_f64 * t4347 * t4674 + t13452 + 2.0_f64 * t13458;
    (t13974, t14001)
}
