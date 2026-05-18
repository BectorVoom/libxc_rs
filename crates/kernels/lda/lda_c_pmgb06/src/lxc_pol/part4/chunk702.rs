//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 702/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk702<F: Float>(t1126: F, t395: F, t2789: F, t301: F, t83: F, t297: F, t4001: F, t1193: F, t934: F) -> (F, F, F, F, F, F) {
    let t4286 = t395 * t1126;
    let t4294 = t83 * t2789 * t301;
    let t4296 = F::new(0.01197423401025461) * t297 * t4294;
    let t4297 = t4001 * t83;
    let t4298 = t4297 * t1193;
    let t4299 = t934 * t301;
    (t4286, t4294, t4296, t4297, t4298, t4299)
}
