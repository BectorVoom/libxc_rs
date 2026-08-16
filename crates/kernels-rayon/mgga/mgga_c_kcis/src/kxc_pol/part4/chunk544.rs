//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 544/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk544(t1131: f64, t2815: f64, t1096: f64, t1092: f64, t10: f64, t251: f64) -> (f64, f64, f64, f64) {
    let t2816 = t1131 * t2815;
    let t2817 = t1096 * t2816;
    let t2818 = t1092 * t2817;
    let t2820 = t10 * t251;
    (t2816, t2817, t2818, t2820)
}
