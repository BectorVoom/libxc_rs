//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 742/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk742(t1588: f64, t1591: f64, t1590: f64, t625: f64, t609: f64) -> (f64, f64, f64) {
    let t4409 = t1588 * t1591;
    let t4413 = 1.0_f64 / t1590 / t625;
    let t4414 = t609 * t4413;
    (t4409, t4413, t4414)
}
