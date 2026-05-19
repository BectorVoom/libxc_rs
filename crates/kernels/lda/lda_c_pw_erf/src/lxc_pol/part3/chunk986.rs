//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 986/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk986<F: Float>(t8419: F, t344: F, t4405: F, t1064: F, t1799: F, t390: F, t40: F, t4383: F, t8438: F, t169: F, t301: F, t5718: F, t717: F) -> (F, F, F, F, F, F) {
    let t11468 = F::new(3.0) * t8419;
    let t11469 = t344 * t4405;
    let t11470 = F::new(12.0) * t11469;
    let t11471 = t1064 * t1799;
    let t11472 = F::new(60.0) * t11471;
    let t11474 = t40 * t4383 * t390;
    let t11475 = F::new(3.0) * t11474;
    let t11476 = F::cast_from(10.526802115419367_f64) * t8438;
    let t11482 = t169 * t717 * t5718 * t301;
    (t11468, t11470, t11472, t11475, t11476, t11482)
}
