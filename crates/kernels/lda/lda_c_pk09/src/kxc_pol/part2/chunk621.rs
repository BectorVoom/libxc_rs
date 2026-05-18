//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 621/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk621<F: Float>(t5012: F, t5117: F, t1285: F, t4998: F, t1329: F, t1468: F, t1387: F, t1472: F, t5039: F, t5045: F, t5068: F, t1413: F, t1416: F) -> (F, F, F, F, F, F, F, F) {
    let t5119 = F::new(38.978347549160304) * t5117 * t5012;
    let t5121 = F::new(12.992782516386768) * t1285 * t4998;
    let t5122 = t1329 * t1468;
    let t5123 = t5122 * t1387;
    let t5124 = t5123 * t1472;
    let t5126 = F::new(0.9421211958699838) * t5039;
    let t5128 = F::new(0.6280807972466558) * t5045;
    let t5134 = F::new(0.20936026574888528) * t5068;
    let t5139 = t1413 * t1416;
    (t5119, t5121, t5123, t5124, t5126, t5128, t5134, t5139)
}
