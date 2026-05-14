//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 928/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk928<F: Float>(t12211: F, t12215: F, t12219: F, t12223: F, t12227: F, t12229: F, t12234: F, t12239: F, t12241: F, t12243: F, t12245: F, t12247: F, t12249: F, t3860: F, t4738: F, t10056: F, t2967: F, t743: F) -> (F, F, F) {
    let t12250 = t12211 + t12215 + t12219 - t12223 - t12227 - t12229 - t12234 - t12239 - t12241 - t12243 + t12245 - t12247 + t12249;
    let t12251 = t4738 * t3860;
    let t12252 = 32.0 / 45.0 * t12251;
    let t12254 = t10056 * t743 * t2967;
    (t12250, t12252, t12254)
}
