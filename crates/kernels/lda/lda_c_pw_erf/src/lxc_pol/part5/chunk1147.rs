//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1147/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1147<F: Float>(t16239: F, t16245: F, t16253: F, t16261: F, t12083: F, t16514: F, t16516: F, t16520: F, t9437: F, t16537: F, t16600: F, t14992: F, t19134: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21112 = F::new(32.0) / F::new(45.0) * t16239;
    let t21113 = F::new(16.0) / F::new(45.0) * t16245;
    let t21114 = F::new(16.0) / F::new(45.0) * t16253;
    let t21115 = F::new(8.0) / F::new(45.0) * t16261;
    let t21116 = F::new(16.0) / F::new(135.0) * t12083;
    let t21118 = F::new(16.0) / F::new(45.0) * t16514;
    let t21119 = F::new(32.0) / F::new(45.0) * t16516;
    let t21120 = F::new(16.0) / F::new(15.0) * t16520;
    let t21121 = F::new(32.0) / F::new(1215.0) * t9437;
    let t21123 = F::new(16.0) / F::new(45.0) * t16537;
    let t21124 = F::new(4.0) / F::new(45.0) * t16600;
    let t21125 = t21112 + t21113 + t21114 - t21115 + t21116 - F::new(2.0) / F::new(15.0) * t19134 + t21118 - t21119 - t21120 + t21121 - F::new(0.19947266666666666) * t14992 - t21123 + t21124;
    (t21112, t21113, t21114, t21115, t21116, t21118, t21119, t21120, t21121, t21123, t21124, t21125)
}
