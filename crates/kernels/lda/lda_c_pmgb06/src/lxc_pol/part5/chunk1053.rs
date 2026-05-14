//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1053/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1053<F: Float>(t248: F, t686: F, t7402: F, t11132: F, t11133: F, t11136: F, t11140: F, t11141: F, t15015: F, t8755: F, t8759: F, t8760: F, t8762: F, t8769: F, t8774: F, t8779: F, t8787: F, t8794: F, t8798: F) -> (F,) {
    let t21801 = t248 * t7402 * t686;
    let t21803 = -t8755 - t8759 + 3.5089341735807875 * t8760 - 51.94757731704439 * t8762 + t8769 - t8774 + t8779 + t11132 - t11133 - 0.0005696894717424259 * t8787 - t8794 + t21801 - t11136 + t11140 + 3.5089341735807875 * t15015 - t8798 + t11141;
    (t21803,)
}
