//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1183/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1183(t14920: f64, t14960: f64, t14998: f64, t15053: f64, t355: f64, t377: f64, t1817: f64, t3369: f64, t1809: f64, t3358: f64, t3436: f64, t9531: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t15055 = t14920 + t14960 + t14998 + t15053;
    let t15056 = t15055 * t355;
    let t15057 = t15056 * sigma0;
    let t15058 = t15057 * t377;
    let t15061 = t3369 * t1817;
    let t15063 = t1809 * t3358;
    let t15065 = t9531 * t3436;
    (t15058, t15061, t15063, t15065)
}
