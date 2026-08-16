//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2626/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2626(t3795: f64, t54042: f64, t40281: f64, t5293: f64, t12283: f64, t16405: f64, t40167: f64, t820: f64, t1799: f64, t3791: f64, t40138: f64, t5259: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54043 = t54042 * t3795;
    let t54047 = t40281 * t5293;
    let t54059 = t12283 * t16405;
    let t54063 = t40167 * t820;
    let t54068 = t1799 * t3791;
    let t54086 = t40138 * t5259;
    (t54043, t54047, t54059, t54063, t54068, t54086)
}
