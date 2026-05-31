//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1105/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1105<F: Float>(t10031: F, t3977: F, t5155: F, t3974: F, t4515: F, t12900: F, t12902: F, t12903: F, t12907: F, t12909: F, t12913: F, t12915: F, t12919: F, t12923: F, t12925: F, t12927: F) -> (F, F, F, F) {
    let t12928 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t10031;
    let t12929 = t5155 * t3977;
    let t12932 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t3974 * t4515 * t12929;
    let t12933 = -t12900 + t12902 - t12903 + t12907 + t12909 - t12913 - t12915 - t12919 + t12923 + t12925 + t12927 - t12928 + t12932;
    (t12928, t12929, t12932, t12933)
}
