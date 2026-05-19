//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 967/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk967<F: Float>(t2760: F, t2771: F, t312: F, t321: F, t4343: F, t642: F, t1767: F, t2764: F, t2765: F, t295: F, t52: F, t740: F, t933: F, t934: F) -> (F, F, F, F) {
    let t8034 = t2760 * t2771;
    let t8039 = F::cast_from(2.8440036129162336_f64) * t321 * t4343 * t642 * t312;
    let t8043 = F::cast_from(3.8666484793229623_f64) * t2764 * t2765 * t1767 * t295;
    let t8047 = F::cast_from(0.6085382050380247_f64) * t933 * t934 * t740 * t52;
    (t8034, t8039, t8043, t8047)
}
