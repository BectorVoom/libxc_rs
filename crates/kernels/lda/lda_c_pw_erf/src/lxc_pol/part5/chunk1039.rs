//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1039/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1039<F: Float>(t1472: F, t6705: F, t1318: F, t3899: F, t6953: F, t12916: F, t6957: F, t2143: F, t5327: F, t2171: F, t4907: F, t4753: F, t6894: F) -> (F, F, F, F, F, F) {
    let t18492 = t1472 * t6705;
    let t18505 = t1318 * t3899 * t6953;
    let t18510 = t1318 * t12916 * t6957;
    let t18517 = t5327 * t2143;
    let t18519 = t2171 * t4907;
    let t18521 = t4753 * t6894;
    (t18492, t18505, t18510, t18517, t18519, t18521)
}
