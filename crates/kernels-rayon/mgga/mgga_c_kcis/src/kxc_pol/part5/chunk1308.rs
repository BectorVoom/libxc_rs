//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1308/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1308(t1445: f64, t7113: f64, t532: f64, t7119: f64, t1401: f64, t7123: f64, t7142: f64, t1419: f64, t21624: f64, t1650: f64, t167: f64, t1437: f64, t21106: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21631 = t1445 * t7113;
    let t21633 = t532 * t7119;
    let t21635 = t1401 * t7123;
    let t21637 = t1401 * t7142;
    let t21641 = t21624 * t1419;
    let t21655 = t1650 * t167;
    let t21665 = t1437 * t21106;
    (t21631, t21633, t21635, t21637, t21641, t21655, t21665)
}
