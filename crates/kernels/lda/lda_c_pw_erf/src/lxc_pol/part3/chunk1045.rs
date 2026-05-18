//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1045/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1045<F: Float>(t2171: F, t3847: F, t3404: F, t2035: F, t9752: F, t3851: F, t2002: F, t12211: F, t12215: F, t12219: F, t12223: F, t12227: F, t12229: F, t12234: F, t12239: F) -> (F, F, F, F, F, F) {
    let t12241 = F::new(4.0) / F::new(15.0) * t2171 * t3847;
    let t12243 = F::new(4.0) / F::new(9.0) * t2171 * t3404;
    let t12245 = F::new(8.0) / F::new(15.0) * t9752 * t2035;
    let t12247 = F::new(4.0) / F::new(15.0) * t2171 * t3851;
    let t12249 = F::new(8.0) / F::new(15.0) * t9752 * t2002;
    let t12250 = t12211 + t12215 + t12219 - t12223 - t12227 - t12229 - t12234 - t12239 - t12241 - t12243 + t12245 - t12247 + t12249;
    (t12241, t12243, t12245, t12247, t12249, t12250)
}
