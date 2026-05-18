//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1100/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1100<F: Float>(t10313: F, t1967: F, t197: F, t519: F, t11746: F, t5256: F, t518: F, t5210: F, t1322: F, t12299: F, t1329: F, t10474: F, t2007: F) -> (F, F, F, F, F, F) {
    let t12869 = t519 * t10313 * t197 * t1967;
    let t12870 = F::new(8.0) / F::new(81.0) * t12869;
    let t12873 = F::new(16.0) / F::new(3.0) * t519 * t5256 * t11746;
    let t12874 = t5210 * t518;
    let t12876 = F::new(16.0) / F::new(15.0) * t12874 * t1322;
    let t12878 = F::new(16.0) / F::new(15.0) * t12299 * t1329;
    let t12880 = F::new(8.0) / F::new(15.0) * t10474 * t2007;
    (t12870, t12873, t12874, t12876, t12878, t12880)
}
