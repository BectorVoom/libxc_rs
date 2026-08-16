//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1020/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1020(t1377: f64, t3978: f64, t1444: f64, t451: f64, t9: f64, t1362: f64, t486: f64, t3716: f64, t503: f64, t4121: f64, t491: f64, t1363: f64, t3951: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12194 = t3978 * t1377;
    let t12216 = 1.0_f64 / t451 / t1444;
    let t12217 = t9 * t12216;
    let t12229 = t1362 * t1362;
    let t12230 = 1.0_f64 / t12229;
    let t12231 = t486 * t12230;
    let t12234 = 1.0_f64 / t3716 / t503;
    let t12240 = t4121 * sigma2;
    let t12241 = t12240 * t491;
    let t12246 = t3951 * t1363;
    (t12194, t12217, t12231, t12234, t12240, t12241, t12246)
}
