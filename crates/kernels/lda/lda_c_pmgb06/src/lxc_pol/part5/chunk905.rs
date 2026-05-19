//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 905/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk905<F: Float>(t10970: F, t317: F, t321: F, t4001: F, t934: F, t97: F, t1786: F, t27: F, t2767: F, t927: F, t2368: F, t754: F, t936: F) -> (F, F, F) {
    let t10976 = F::cast_from(0.3407285805772476_f64) * t4001 * t321 / t10970 * t317 * t97 * t934;
    let t10980 = t927 * t1786 * t27 * t2767;
    let t10984 = t2368 * t754 * t97 * t936;
    (t10976, t10980, t10984)
}
