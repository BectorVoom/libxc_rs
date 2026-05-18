//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1166/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1166<F: Float>(t21332: F, t11038: F, t12558: F, t12638: F, t21322: F, t21323: F, t21324: F, t21325: F, t21326: F, t21327: F, t21329: F, t21330: F, t21331: F) -> (F, F) {
    let t21333 = F::new(4.0) / F::new(15.0) * t21332;
    let t21334 = -t21322 - t21323 - t21324 + t21325 + t21326 - t21327 - t21329 - t11038 - t12558 + t21330 + t21331 - t12638 - t21333;
    (t21333, t21334)
}
