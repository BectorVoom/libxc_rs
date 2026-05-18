//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 500/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk500<F: Float>(t2824: F, t467: F, t452: F, t2758: F, t471: F, t1782: F, t2778: F) -> (F, F, F, F) {
    let t2825 = t467 * t2824;
    let t2826 = t2825 * t452;
    let t2829 = t471 * t2758;
    let t2832 = t2778 * t1782;
    (t2825, t2826, t2829, t2832)
}
