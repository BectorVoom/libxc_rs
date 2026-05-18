//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 944/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk944<F: Float>(t4619: F, t464: F, t1894: F, t3213: F, t3055: F, t802: F, t1464: F, t524: F, t2911: F, t3357: F, t13372: F, t1575: F, t2918: F) -> (F, F, F, F, F, F, F) {
    let t13933 = t4619 * t464;
    let t13948 = t3213 * t1894;
    let t13949 = F::new(2.0) / F::new(135.0) * t13948;
    let t14015 = t802 * t3055;
    let t14016 = t14015 / F::new(45.0);
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    let t14127 = F::new(0.03199259259259259) * t13372;
    let t14152 = t1575 * t2918;
    (t13933, t13949, t14016, t14106, t14110, t14127, t14152)
}
