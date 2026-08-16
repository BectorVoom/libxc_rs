//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 716/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk716(t1398: f64, t4142: f64, t1397: f64, t3738: f64, t1394: f64, t1396: f64, t3805: f64, t1395: f64, t1017: f64, t3751: f64, t86: f64, t3797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4143 = t4142 * t1398;
    let t4145 = t3738 * t1397;
    let t4146 = t1394 * t4145;
    let t4148 = t1396 * t3805;
    let t4149 = t1395 * t4148;
    let t4150 = t1394 * t4149;
    let t4153 = t86 * t1017 * t3751;
    let t4154 = t1396 * t3797;
    (t4143, t4145, t4146, t4148, t4149, t4150, t4153, t4154)
}
