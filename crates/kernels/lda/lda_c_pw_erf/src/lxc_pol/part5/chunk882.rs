//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 882/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk882<F: Float>(t1759: F, t4295: F, t1059: F, t2948: F, t260: F, t34: F, t343: F, t262: F, t3154: F, t344: F, t339: F, t311: F) -> (F, F, F, F, F, F, F, F) {
    let t8301 = t1759 * t4295;
    let t8303 = t1059 * t2948;
    let t8315 = F::new(1.0) / t260;
    let t8327 = t34 * t343;
    let t8334 = F::new(1.0) / t262;
    let t8356 = F::new(16.0) * t344 * t3154;
    let t8357 = t339 * t3154;
    let t8359 = t311 * t311;
    (t8301, t8303, t8315, t8327, t8334, t8356, t8357, t8359)
}
