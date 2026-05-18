//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1006/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1006<F: Float>(t519: F, t6427: F, t9304: F, t13432: F, t6464: F, t1325: F, t3859: F, t6468: F, t2388: F, t571: F, t9313: F, t1518: F, t185: F, t2472: F) -> (F, F, F, F, F) {
    let t16042 = t519 * t9304 * t6427;
    let t16050 = t519 * t13432 * t6464;
    let t16053 = t1325 * t3859 * t6468;
    let t16058 = t571 * t9313 * t2388;
    let t16065 = t185 * t1518 * t2472;
    (t16042, t16050, t16053, t16058, t16065)
}
