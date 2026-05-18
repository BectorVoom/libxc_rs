//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 582/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk582<F: Float>(t12: F, t155: F, t3134: F, t1512: F, t460: F, t1083: F, t337: F, t2938: F, t44: F, t131: F, t178: F, t436: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t3136 = t3134 * t155 / F::new(30.0);
    let t3138 = t1512 * t460 / F::new(10.0);
    let t3139 = t337 * t1083;
    let t3144 = piecewise3::<f64>(t13, F::new(0.0), F::new(2.0) * t12 * t2938 + F::new(6.0) * t3139);
    let t3145 = t3144 * t44;
    let t3146 = t3145 * t131;
    let t3148 = t3146 * t178 / F::new(30.0);
    let t3149 = t1512 * t436;
    (t3136, t3138, t3139, t3145, t3146, t3148, t3149)
}
