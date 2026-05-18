//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 651/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk651<F: Float>(t2262: F, t707: F, t2266: F, t1773: F, t909: F, t123: F, t2164: F, t317: F, t740: F, t117: F, t2360: F, t315: F) -> (F, F, F, F, F) {
    let t5590 = F::new(0.039914113367515366) * t707 * t2262;
    let t5591 = t707 * t2266;
    let t5593 = t1773 * t909;
    let t5601 = F::new(0.10809180959278285) * t123 * t740 * t2164 * t317;
    let t5610 = F::new(0.017961351015381915) * t123 * t315 * t2360 * t117;
    (t5590, t5591, t5593, t5601, t5610)
}
