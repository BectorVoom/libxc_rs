//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1011/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1011(t1392: f64, t86: f64, t9526: f64, t1398: f64, t4142: f64, t4145: f64, t2820: f64, t4158: f64) -> (f64, f64, f64, f64) {
    let t11881 = t86 * t9526 * t1392;
    let t11882 = t11881 * t1398;
    let t11884 = t4142 * t4145;
    let t11913 = t86 * t2820 * t4158;
    (t11881, t11882, t11884, t11913)
}
