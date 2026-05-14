//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 971/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk971<F: Float>(t10953: F, t169: F, t289: F, t1089: F, t39: F, t343: F, t678: F, t2817: F, t5: F, t168: F, t286: F, t1112: F, t1184: F, t1187: F, t483: F, t1131: F, t2829: F) -> (F, F, F, F, F, F, F) {
    let t10956 = 0.031835665774679375 * t169 * t289 * t10953;
    let t10961 = t39 * t1089;
    let t10963 = t343 * t678;
    let t10967 = t5 * t2817;
    let t10970 = 0.9106331049773876 * t168 * t10967 * t286;
    let t10973 = t1184 * t1112 * t483 * t1187;
    let t10976 = t2829 * t1131 * t1187;
    (t10956, t10961, t10963, t10967, t10970, t10973, t10976)
}
