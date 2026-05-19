//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 650/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk650<F: Float>(t372: F, t4977: F, t310: F, t1337: F, t1311: F, t4998: F, t309: F, t5420: F, t1339: F, t1519: F, t318: F, t5308: F) -> (F, F, F, F, F) {
    let t5674 = t372 * t4977;
    let t5675 = t310 * t5674;
    let t5677 = F::cast_from(0.04115066352984959_f64) * t1337 * t5675;
    let t5679 = F::cast_from(1.2536914064583544_f64) * t1311 * t4998;
    let t5680 = t5420 * t309;
    let t5681 = t5680 * t1339;
    let t5683 = t318 * t1519;
    let t5684 = t5683 * t5308;
    (t5677, t5679, t5681, t5683, t5684)
}
