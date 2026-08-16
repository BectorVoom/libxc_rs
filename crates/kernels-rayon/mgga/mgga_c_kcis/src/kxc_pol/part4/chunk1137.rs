//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1137/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1137(t278: f64, t1000: f64, t992: f64, t1071: f64, t1003: f64, t1646: f64, t829: f64, t2887: f64, t2844: f64, t14051: f64, t1001: f64, t286: f64, t110: f64, t1705: f64) -> (f64, f64, f64, f64) {
    let t288 = 0.0_f64 < t278;
    let t14400 = t992 * t1000;
    let t14401 = t14400 * t1071;
    let t14402 = t1646 * t1003;
    let t14403 = t14402 * t829;
    let t14404 = t14401 * t14403;
    let t14407 = t2887 * t1000;
    let t14408 = t14407 * t2844;
    let t14409 = t14408 * t14403;
    let t14413 = piecewise3(t288, t14051, -t14051);
    let t14414 = t1001 * t14413;
    let t14415 = t286 * t14414;
    let t14422 = t110 * t1705;
    (t14404, t14409, t14415, t14422)
}
