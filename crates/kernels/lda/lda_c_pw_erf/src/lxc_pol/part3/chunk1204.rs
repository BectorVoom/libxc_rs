//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1204/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1204<F: Float>(t1318: F, t13294: F, t4868: F, t3859: F, t4637: F, t519: F, t4615: F, t5237: F, t11691: F, t5256: F, t1446: F, t5251: F) -> (F, F, F, F, F) {
    let t14188 = F::new(8.0) / F::new(9.0) * t1318 * t4868 * t13294;
    let t14190 = t519 * t3859 * t4637;
    let t14191 = F::new(16.0) / F::new(45.0) * t14190;
    let t14193 = t519 * t5237 * t4615;
    let t14194 = F::new(16.0) / F::new(9.0) * t14193;
    let t14197 = F::new(8.0) / F::new(9.0) * t519 * t5256 * t11691;
    let t14199 = F::new(32.0) / F::new(27.0) * t1446 * t5251;
    (t14188, t14191, t14194, t14197, t14199)
}
