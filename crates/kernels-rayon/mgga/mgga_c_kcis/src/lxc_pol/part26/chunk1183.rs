//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1183/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1183(t4413: f64, t6136: f64, t12857: f64, t2093: f64, t4188: f64, t7267: f64, t1505: f64, t22298: f64, t38630: f64, t7042: f64, t12321: f64, t6922: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54605 = t6136 * t4413;
    let t54624 = t2093 * t12857;
    let t54732 = t7267 * t4188;
    let t54773 = t22298 * t1505;
    let t58540 = t7042 * t38630;
    let t58599 = t12321 * t6922;
    (t54605, t54624, t54732, t54773, t58540, t58599)
}
