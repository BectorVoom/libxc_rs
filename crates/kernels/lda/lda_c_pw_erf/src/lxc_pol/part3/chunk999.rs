//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 999/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk999<F: Float>(t163: F, t169: F, t2198: F, t717: F, t299: F, t5433: F, t1440: F, t2186: F, t3545: F, t519: F, t1318: F, t2192: F, t9432: F) -> (F, F, F, F) {
    let t11666 = t169 * t717 * t2198 * t163;
    let t11667 = F::cast_from(0.07184540406152766_f64) * t11666;
    let t11670 = t169 * t299 * t5433 * t163;
    let t11675 = F::new(4.0) / F::new(15.0) * t519 * t1440 * t2186 * t3545;
    let t11677 = t1318 * t9432 * t2192;
    (t11667, t11670, t11675, t11677)
}
