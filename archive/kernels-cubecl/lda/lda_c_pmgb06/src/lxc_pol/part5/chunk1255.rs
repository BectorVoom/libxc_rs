//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1255/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1255<F: Float>(t14016: F, t21066: F, t21068: F, t21069: F, t21071: F, t21074: F, t21078: F, t21080: F, t21082: F, t21083: F, t21086: F, t21088: F) -> F {
    let t22044 = -t21066 + t14016 - t21068 + t21069 + t21071 + t21074 + t21078 - t21080 - t21082 + t21083 + t21086 + t21088;
    t22044
}
