//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 893/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk893<F: Float>(t13432: F, t519: F, t6464: F, t1325: F, t3859: F, t6468: F, t2388: F, t571: F, t9313: F, t1518: F, t185: F, t2472: F, t6374: F, t9278: F, t14240: F, t6384: F) -> (F, F, F, F, F, F) {
    let t16050 = t519 * t13432 * t6464;
    let t16053 = t1325 * t3859 * t6468;
    let t16058 = t571 * t9313 * t2388;
    let t16065 = t185 * t1518 * t2472;
    let t16069 = t571 * t9278 * t6374;
    let t16072 = t571 * t14240 * t6384;
    (t16050, t16053, t16058, t16065, t16069, t16072)
}
