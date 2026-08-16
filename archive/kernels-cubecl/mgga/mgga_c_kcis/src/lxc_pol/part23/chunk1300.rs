//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1300/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1300<F: Float>(t27556: F, t28778: F, t3978: F, t7984: F, t18155: F, t2256: F, t2260: F, t27583: F, t28708: F, t54581: F, t7968: F, t7971: F, t94472: F, t94474: F, t94483: F, t94901: F, t95045: F, t95052: F, t99013: F, t99348: F) -> F {
    let t99392 = F::cast_from(0.30918233506944444444e-4_f64) * t27556 * t28778;
    let t99403 = t3978 * t7984;
    let t99407 = -F::cast_from(0.51485339506172839507e-4_f64) * t95045 - F::cast_from(0.3861400462962962963e-4_f64) * t95052 - F::cast_from(0.51588271604938271604e-3_f64) * t94472 - F::cast_from(0.15476481481481481481e-2_f64) * t94474 + t99392 + F::cast_from(0.69505208333333333334e-3_f64) * t99013 * t7971 + F::cast_from(0.15476481481481481481e-2_f64) * t94483 - F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t99348 - F::cast_from(0.185671721767578125e-4_f64) * t94901 * t28708 - F::cast_from(0.34752604166666666667e-3_f64) * t54581 * t2256 * t2260 + F::cast_from(0.30891203703703703704e-3_f64) * t27583 * t99403 * t18155;
    t99407
}
