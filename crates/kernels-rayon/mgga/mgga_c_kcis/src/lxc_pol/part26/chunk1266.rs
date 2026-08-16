//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1266/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1266(t12844: f64, t27583: f64, t28748: f64, t27566: f64, t28720: f64, t27567: f64, t99422: f64, t18210: f64, t28810: f64, t7978: f64, t99023: f64, t98743: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99556 = 0.7722800925925925926e-4_f64 * t27583 * t12844 * t28748;
    let t99565 = t28720 * t27566;
    let t99578 = 0.10306077835648148148e-4_f64 * t27567 * t99422;
    let t99591 = 0.46336805555555555556e-3_f64 * t7978 * t18210 * t28810;
    let t99593 = 0.23168402777777777778e-3_f64 * t7978 * t99023;
    let t99600 = 0.15476481481481481481e-2_f64 * t98743;
    (t99556, t99565, t99578, t99591, t99593, t99600)
}
