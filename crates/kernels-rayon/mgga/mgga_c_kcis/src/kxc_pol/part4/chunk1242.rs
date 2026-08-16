//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1242/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1242(t16034: f64, t3786: f64, t10338: f64, t1988: f64, t1890: f64, t2323: f64) -> (f64, f64, f64) {
    let t16035 = t3786 * t16034;
    let t16038 = t10338 * t1988;
    let t16046 = t2323 * t1890;
    (t16035, t16038, t16046)
}
