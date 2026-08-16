//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 935/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk935(t1045: f64, t14196: f64, t4647: f64, t3255: f64, t4639: f64, t4644: f64, t3074: f64, t4848: f64, t4642: f64, t313: f64, t4670: f64, t934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14198 = t14196 * t4647 * t1045;
    let t14202 = 0.19711289e-2_f64 * t3255 * t4639;
    let t14204 = 0.26281718666666666666e-2_f64 * t3255 * t4644;
    let t14205 = t4848 * t3074;
    let t14206 = t4642 * t14205;
    let t14209 = t313 * t4670;
    let t14210 = t14209 * t934;
    (t14198, t14202, t14204, t14205, t14206, t14210)
}
