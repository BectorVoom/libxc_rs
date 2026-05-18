//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1282/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1282<F: Float>(t1313: F, t519: F, t542: F, t7655: F, t1326: F, t22759: F, t22915: F, t22918: F, t22921: F, t22926: F, t22931: F, t22934: F, t22937: F, t22940: F, t22944: F, t22945: F, t22950: F, t22954: F) -> (F, F, F) {
    let t22958 = F::new(4.0) / F::new(45.0) * t519 * t1313 * t7655 * t542;
    let t22961 = F::new(8.0) / F::new(45.0) * t519 * t1326 * t22759;
    let t22962 = -t22915 - t22918 + t22921 + t22926 - t22931 - t22934 + t22937 + t22940 - t22944 + t22945 - t22950 + t22954 - t22958 - t22961;
    (t22958, t22961, t22962)
}
