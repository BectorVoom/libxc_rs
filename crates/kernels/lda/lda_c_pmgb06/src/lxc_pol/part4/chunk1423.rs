//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1423/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1423<F: Float>(t17279: F, t17282: F, t17284: F, t17286: F, t17288: F, t17290: F, t17296: F, t17300: F, t17304: F, t17305: F, t17306: F, t17307: F, t17308: F, t17367: F, t17369: F) -> F {
    let t18300 = t17279 - t17282 - t17284 - t17286 + t17288 - t17290 - t17296 - t17300 - t17304 - t17305 - t17306 - t17307 - t17308 + t17367 + t17369;
    t18300
}
