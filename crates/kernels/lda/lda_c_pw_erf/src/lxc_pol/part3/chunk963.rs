//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 963/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk963<F: Float>(t1415: F, t1432: F, t256: F, t1427: F, t3946: F, t3949: F, t656: F, t3933: F, t1: F, t3921: F, t4166: F, t119: F, t1426: F, t3920: F) -> (F, F, F, F, F, F) {
    let t11053 = t1415 * t1432 * t256;
    let t11055 = t3946 * t1427;
    let t11057 = t3949 * t656;
    let t11063 = F::new(8.0) / F::new(9.0) * t3933 * t656;
    let t11065 = t4166 * t1 * t3921;
    let t11069 = F::cast_from(0.006061752703703704_f64) * t3920 * t119 * t1426;
    (t11053, t11055, t11057, t11063, t11065, t11069)
}
