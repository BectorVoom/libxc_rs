//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 614/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk614<F: Float>(t1122: F, t4549: F, t2148: F, t980: F, t968: F, t2142: F, t273: F, t698: F, t959: F, t3941: F, t3945: F, t3948: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4550 = t4549 * t1122;
    let t4552 = t2148 * t980;
    let t4554 = t2148 * t968;
    let t4556 = t2142 * t273;
    let t4558 = F::cast_from(1.1696447245269292_f64) * t4556 * t698;
    let t4559 = t2148 * t959;
    let t4568 = F::cast_from(12.0_f64) * t3941;
    let t4569 = F::cast_from(48.0_f64) * t3945;
    let t4570 = F::cast_from(80.0_f64) * t3948;
    (t4550, t4552, t4554, t4556, t4558, t4559, t4568, t4569, t4570)
}
