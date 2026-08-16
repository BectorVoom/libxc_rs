//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 380/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk380(t1307: f64, t1430: f64, t1060: f64, t323: f64, t526: f64, t251: f64, t461: f64) -> (f64, f64, f64) {
    let t1431 = t1430 * t1307;
    let t1436 = 0.7925e-3_f64 * t323 * t1060 * t526;
    let t1437 = t251 * t461;
    (t1431, t1436, t1437)
}
