//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1145/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1145(t4962: f64, t9938: f64, t991: f64, t1071: f64, t1704: f64, t2630: f64, t2894: f64, t1000: f64, t4951: f64, t1003: f64, t4621: f64, t13475: f64, t4947: f64) -> (f64, f64, f64, f64) {
    let t14536 = t9938 * t4962;
    let t14538 = t991 * t14536 / 432.0_f64;
    let t14542 = t1704 * t1071 * t2630;
    let t14543 = t2894 * t14542;
    let t14546 = t4951 * t1000;
    let t14547 = t4621 * t1003;
    let t14548 = t14546 * t14547;
    let t14551 = t4947 * t13475;
    (t14538, t14543, t14548, t14551)
}
