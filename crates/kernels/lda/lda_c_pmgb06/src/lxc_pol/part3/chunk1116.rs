//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1116/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1116<F: Float>(t10087: F, t10089: F, t1467: F, t5305: F, t1972: F, t3195: F, t3235: F, t3239: F, t1963: F, t3177: F, t1420: F, t4615: F) -> (F, F, F, F, F, F, F, F) {
    let t13257 = t10087 / F::new(45.0);
    let t13258 = F::new(2.0) / F::new(45.0) * t10089;
    let t13260 = t5305 * t1467 / F::new(9.0);
    let t13262 = t1972 * t3195 / F::new(15.0);
    let t13264 = t1972 * t3235 / F::new(15.0);
    let t13266 = t1972 * t3239 / F::new(9.0);
    let t13268 = t3177 * t1963 / F::new(15.0);
    let t13270 = t1420 * t4615 / F::new(15.0);
    (t13257, t13258, t13260, t13262, t13264, t13266, t13268, t13270)
}
