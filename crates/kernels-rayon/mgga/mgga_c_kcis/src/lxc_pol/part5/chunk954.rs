//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 954/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk954(t1071: f64, t1130: f64, t3209: f64, t982: f64, t169: f64, t2843: f64, t2844: f64, t1131: f64, t3201: f64, t251: f64, t88: f64, t304: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9432 = t1130 * t1071;
    let t9438 = t3209 * t982;
    let t9494 = 1.0_f64 / t2843 / t169;
    let t9512 = t1130 * t2844;
    let t9517 = t3201 * t1131;
    let t9526 = t88 * t251;
    let t9528 = t86 * t9526 * t304;
    (t9432, t9438, t9494, t9512, t9517, t9526, t9528)
}
