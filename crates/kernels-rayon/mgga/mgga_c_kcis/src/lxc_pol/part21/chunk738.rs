//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 738/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk738(t7754: f64, t8072: f64, t389: f64, t4999: f64, t1096: f64, t1813: f64, t1021: f64, t1817: f64, t8067: f64, t8070: f64) -> (f64, f64, f64, f64, f64) {
    let t8073 = t7754 * t8072;
    let t8075 = t4999 * t389;
    let t8077 = t1096 * t1813;
    let t8079 = t1021 * t1817;
    let t8081 = t8067 / 16.0_f64 - t8070 / 16.0_f64 + t8073 / 24.0_f64 - t8075 / 128.0_f64 + t8077 / 128.0_f64 - t8079 / 96.0_f64;
    (t8073, t8075, t8077, t8079, t8081)
}
