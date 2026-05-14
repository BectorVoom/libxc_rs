//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1070/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1070<F: Float>(t14549: F, t10873: F, t10876: F, t10877: F, t10881: F, t10886: F, t11710: F, t118: F, t14527: F, t14529: F, t14533: F, t14536: F, t14539: F, t14541: F, t14544: F, t14545: F, t14547: F) -> (F,) {
    let t14550 = 0.1890324433388467 * t14549;
    let t14551 = -0.031505407223141116 * t10873 + t10876 + 0.031505407223141116 * t10877 + 0.008980675507690957 * t10881 - t10886 + 0.0878110494085338 * t14527 - 0.031505407223141116 * t14529 - 0.031505407223141116 * t11710 * t118 - 0.09451622166942335 * t14533 - t14536 + 0.02694202652307287 * t14539 + 0.09451622166942335 * t14541 - t14544 - 0.1890324433388467 * t14545 + 0.09451622166942335 * t14547 + t14550;
    (t14551,)
}
