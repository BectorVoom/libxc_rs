//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 715/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk715(t4135: f64, t4136: f64, t1395: f64, t1464: f64, t1392: f64, t2820: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t4137 = t4135 * t4136;
    let t4138 = t1395 * t4137;
    let t4139 = t1464 * t4138;
    let t4142 = t86 * t2820 * t1392;
    (t4137, t4138, t4139, t4142)
}
