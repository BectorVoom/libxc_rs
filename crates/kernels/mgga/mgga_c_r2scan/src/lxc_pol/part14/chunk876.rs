//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 876/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk876<F: Float>(t5109: F, t7963: F, t2559: F, t7494: F, t2124: F, t2550: F, t7944: F, t1551: F, t2526: F, t277: F, t495: F, t360: F) -> (F, F, F, F, F, F, F) {
    let t7964 = t5109 * t7963;
    let t7968 = F::new(0.12805040077930161442e0) * t7494 * t2559;
    let t7970 = t2124 * t2550 * t7944;
    let t7974 = t2124 * t2550 * t1551;
    let t7977 = t277 * t2526;
    let t7978 = t7977 * t495;
    let t7979 = t360 * t7978;
    (t7964, t7968, t7970, t7974, t7977, t7978, t7979)
}
