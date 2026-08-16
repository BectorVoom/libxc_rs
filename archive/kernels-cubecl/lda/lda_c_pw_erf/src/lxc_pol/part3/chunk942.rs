//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 942/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk942<F: Float>(t10605: F, t1487: F, t571: F, t3715: F, t4062: F, t1472: F, t4059: F, t4063: F, t1325: F, t3731: F, t3787: F, t1340: F, t3783: F, t519: F) -> (F, F, F, F, F, F) {
    let t10607 = t571 * t10605 * t1487;
    let t10610 = t571 * t4062 * t3715;
    let t10612 = t1472 * t4059;
    let t10614 = t1472 * t4063;
    let t10617 = t1325 * t3787 * t3731;
    let t10620 = t519 * t3783 * t1340;
    (t10607, t10610, t10612, t10614, t10617, t10620)
}
