//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1365/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1365(t1517: f64, t1518: f64, t16073: f64, t5976: f64, t1392: f64, t1455: f64, t5441: f64, t16082: f64, t16078: f64, t14955: f64, t5977: f64, t5969: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17565 = t1517 * t1518;
    let t17568 = t5976 * t16073;
    let t17571 = t1392 * t1455;
    let t17572 = t17571 * t5441;
    let t17575 = t5976 * t16082;
    let t17578 = t5976 * t16078;
    let t17583 = t14955 * t5977;
    let t17586 = 0.5895802469135802469e-1_f64 * t14955 * t5969;
    (t17565, t17568, t17572, t17575, t17578, t17583, t17586)
}
