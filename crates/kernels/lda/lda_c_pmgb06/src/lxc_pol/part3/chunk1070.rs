//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1070/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1070<F: Float>(t1444: F, t5282: F, t10293: F, t493: F, t5281: F, t12672: F, t12676: F, t12678: F, t12682: F, t12686: F, t12690: F, t12696: F, t12700: F, t12704: F, t12708: F) -> (F, F, F) {
    let t12710 = t1444 * t5282 / F::new(9.0);
    let t12713 = t493 * t10293 * t5281 / F::new(9.0);
    let t12714 = -t12672 + t12676 - t12678 - t12682 + t12686 + t12690 - t12696 - t12700 - t12704 - t12708 - t12710 - t12713;
    (t12710, t12713, t12714)
}
