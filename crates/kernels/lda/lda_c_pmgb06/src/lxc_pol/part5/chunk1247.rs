//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1247/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1247<F: Float>(t591: F, t7975: F, t13440: F, t13444: F, t13447: F, t13450: F, t20784: F, t20786: F, t20789: F, t20791: F, t20792: F, t20794: F, t20797: F) -> F {
    let t22018 = t7975 * t591;
    let t22021 = -t20784 - t20786 - t20789 - t20791 - t20792 - t20794 + t13440 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t22018 + t13444 + t13447 + F::cast_from(0.547_f64) * t13450 + t20797;
    t22021
}
