//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 653/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk653<F: Float>(t5045: F, t5068: F, t1560: F, t305: F, t304: F, t5420: F, t1625: F, t309: F, t310: F, t4977: F, t1642: F, t131: F, t623: F) -> (F, F, F, F, F, F, F) {
    let t5733 = F::cast_from(7.919542066025344_f64) * t5045;
    let t5739 = F::cast_from(2.6398473553417814_f64) * t5068;
    let t5747 = t1560 * t305;
    let t5751 = t304 * t5420;
    let t5752 = t5751 * t1625;
    let t5755 = t309 * t310 * t4977;
    let t5757 = t1642 * t5755 / F::cast_from(6.0_f64);
    let t5759 = t309 * t131 * t623;
    (t5733, t5739, t5747, t5752, t5755, t5757, t5759)
}
