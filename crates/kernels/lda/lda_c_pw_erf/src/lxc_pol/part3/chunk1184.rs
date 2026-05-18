//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1184/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1184<F: Float>(t1381: F, t3974: F, t3976: F, t5155: F, t13115: F, t13116: F, t593: F, t10027: F, t5162: F, t12475: F, t12492: F, t5147: F) -> (F, F, F, F) {
    let t13952 = F::new(8.0) / F::new(15.0) * t3974 * t3976 * t5155 * t1381;
    let t13956 = F::new(32.0) / F::new(15.0) * t13115 * t3976 * t13116 * t593;
    let t13958 = F::new(32.0) / F::new(15.0) * t10027 * t5162;
    let t13961 = F::new(32.0) / F::new(9.0) * t12475 * t5147 * t12492;
    (t13952, t13956, t13958, t13961)
}
