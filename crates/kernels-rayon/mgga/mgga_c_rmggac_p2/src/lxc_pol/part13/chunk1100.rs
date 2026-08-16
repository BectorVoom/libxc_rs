//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1100/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1100(t874: f64, t9486: f64, t352: f64, t2447: f64, t4616: f64, t876: f64, t42023: f64, t42026: f64, t4905: f64, t9540: f64, t42044: f64, t42057: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43970 = t874 * t9486;
    let t43971 = t43970 * t352;
    let t43974 = t4616 * t2447;
    let t43975 = t43974 * t876;
    let t43978 = 0.162600798888400151e-2_f64 * t42023;
    let t43979 = 0.162600798888400151e-2_f64 * t42026;
    let t43981 = t9540 * t4905;
    let t43987 = 0.11918087970123395032e-3_f64 * t42044;
    let t43990 = 0.87811105813667929469e0_f64 * t42057;
    (t43971, t43975, t43978, t43979, t43981, t43987, t43990)
}
