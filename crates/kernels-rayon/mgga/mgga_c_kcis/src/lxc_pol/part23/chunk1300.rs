//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1300/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1300(t27556: f64, t28778: f64, t3978: f64, t7984: f64, t18155: f64, t2256: f64, t2260: f64, t27583: f64, t28708: f64, t54581: f64, t7968: f64, t7971: f64, t94472: f64, t94474: f64, t94483: f64, t94901: f64, t95045: f64, t95052: f64, t99013: f64, t99348: f64) -> f64 {
    let t99392 = 0.30918233506944444444e-4_f64 * t27556 * t28778;
    let t99403 = t3978 * t7984;
    let t99407 = -0.51485339506172839507e-4_f64 * t95045 - 0.3861400462962962963e-4_f64 * t95052 - 0.51588271604938271604e-3_f64 * t94472 - 0.15476481481481481481e-2_f64 * t94474 + t99392 + 0.69505208333333333334e-3_f64 * t99013 * t7971 + 0.15476481481481481481e-2_f64 * t94483 - 0.46377350260416666667e-4_f64 * t7968 * t99348 - 0.185671721767578125e-4_f64 * t94901 * t28708 - 0.34752604166666666667e-3_f64 * t54581 * t2256 * t2260 + 0.30891203703703703704e-3_f64 * t27583 * t99403 * t18155;
    t99407
}
