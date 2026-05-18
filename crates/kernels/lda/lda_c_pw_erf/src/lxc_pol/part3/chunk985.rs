//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 985/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk985<F: Float>(t11432: F, t11458: F, t59: F, t40: F, t87: F, t1765: F, t2948: F, t1077: F, t4393: F, t11388: F, t11390: F, t11392: F, t11398: F, t11399: F, t11402: F, t11404: F, t11406: F, t8386: F, t8389: F, t8393: F, t8397: F, t8400: F, t8403: F, t8405: F) -> (F, F, F, F, F) {
    let t11460 = (t11432 + t11458) * t59;
    let t11462 = t40 * t11460 * t87;
    let t11463 = t1765 * t2948;
    let t11464 = F::new(103.89453539625518) * t11463;
    let t11465 = t4393 * t1077;
    let t11466 = F::new(3.5089340384731225) * t11465;
    let t11467 = t8386 - t11388 + t11390 - t11392 - t8389 - t8393 + t8397 - t8400 - F::new(0.41076328840066667) * t8403 + F::new(2.0538164420033334) * t8405 - t11398 + F::new(3.1636214830824234) * t11399 + t11402 + t11404 - t11406 + t11462 + t11464 + t11466;
    (t11460, t11462, t11464, t11466, t11467)
}
