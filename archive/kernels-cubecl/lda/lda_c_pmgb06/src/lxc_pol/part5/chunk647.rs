//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 647/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk647<F: Float>(t1983: F, t5499: F, t122: F, t2116: F, t569: F, t107: F, t2164: F, t410: F, t199: F, t2174: F, t718: F, t868: F) -> (F, F, F, F, F) {
    let t5500 = t5499 * t1983;
    let t5514 = F::cast_from(0.039794582218349216_f64) * t122 * t569 * t2116;
    let t5517 = F::cast_from(1.1389037339096726_f64) * t107 * t410 * t2164;
    let t5518 = t2174 * t199;
    let t5520 = t718 * t868;
    (t5500, t5514, t5517, t5518, t5520)
}
