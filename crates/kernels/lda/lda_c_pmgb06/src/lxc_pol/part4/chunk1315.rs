//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1315/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1315<F: Float>(t17287: F, t1444: F, t6756: F, t17261: F, t17262: F, t17263: F, t17264: F, t17265: F, t17266: F, t17268: F, t17272: F, t17275: F, t17279: F, t17282: F, t17284: F, t17286: F) -> (F, F, F) {
    let t17288 = F::new(4.0) / F::new(81.0) * t17287;
    let t17290 = F::new(2.0) / F::new(45.0) * t1444 * t6756;
    let t17291 = -t17261 - t17262 - t17263 - t17264 - t17265 - t17266 + t17268 + t17272 + t17275 + t17279 - t17282 - t17284 - t17286 + t17288 - t17290;
    (t17288, t17290, t17291)
}
