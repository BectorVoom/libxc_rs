//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 976/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk976<F: Float>(t8191: F, t8195: F, t1765: F, t2942: F, t1070: F, t1775: F, t1067: F, t1799: F, t8197: F, t8199: F, t8204: F, t8206: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11333 = F::new(48.0) * t8191;
    let t11334 = F::new(72.0) * t8195;
    let t11335 = t1765 * t2942;
    let t11336 = F::new(3.5089340384731225) * t11335;
    let t11337 = t1070 * t1775;
    let t11338 = F::new(96.0) * t11337;
    let t11339 = t1067 * t1799;
    let t11340 = F::new(36.0) * t11339;
    let t11341 = F::new(480.0) * t8197;
    let t11342 = F::new(144.0) * t8199;
    let t11343 = F::new(240.0) * t8204;
    let t11344 = F::new(12.0) * t8206;
    (t11333, t11334, t11336, t11338, t11340, t11341, t11342, t11343, t11344)
}
