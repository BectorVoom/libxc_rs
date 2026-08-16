//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1284/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1284(t17326: f64, t27494: f64, t12345: f64, t4310: f64, t8186: f64, t2104: f64, t30409: f64, t4312: f64, t6176: f64, t97997: f64, t27563: f64, t28727: f64) -> (f64, f64, f64, f64, f64) {
    let t98959 = 2.0_f64 * t27494 * t17326;
    let t98963 = 6.0_f64 * t12345 * t8186 * t4310;
    let t98971 = t6176 * t30409 * t2104 * t4312;
    let t98978 = 0.15476481481481481481e-2_f64 * t97997;
    let t98986 = 0.61782407407407407408e-3_f64 * t28727 * t27563;
    (t98959, t98963, t98971, t98978, t98986)
}
