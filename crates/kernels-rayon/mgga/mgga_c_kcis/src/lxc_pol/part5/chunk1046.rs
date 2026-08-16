//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1046/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1046(t3820: f64, t509: f64, t1409: f64, t1897: f64, t1098: f64, t5483: f64, t1992: f64, t3251: f64, t1958: f64, t1317: f64, t5523: f64, t16048: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16411 = t509 * t3820;
    let t16416 = t1409 * t1897;
    let t16436 = 0.19711289e-2_f64 * t1098 * t5483;
    let t16441 = t3251 * t1992;
    let t16500 = t3820 * t1958;
    let t16503 = t1317 * t5523;
    let t16523 = 0.18344444444444444444e-2_f64 * t16048;
    (t16411, t16416, t16436, t16441, t16500, t16503, t16523)
}
