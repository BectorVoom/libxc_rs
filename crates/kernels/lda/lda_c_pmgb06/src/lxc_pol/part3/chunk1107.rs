//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1107/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1107<F: Float>(t2002: F, t3255: F, t13144: F, t13149: F, t13151: F, t13153: F, t13156: F, t13158: F, t13160: F, t13162: F, t13165: F, t13167: F, t13170: F) -> (F, F) {
    let t13172 = t2002 * t3255 / F::new(45.0);
    let t13173 = -t13144 + t13149 + t13151 + t13153 + t13156 + t13158 + t13160 + t13162 + t13165 + t13167 + t13170 + t13172;
    (t13172, t13173)
}
