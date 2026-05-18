//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1126/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1126<F: Float>(t1440: F, t15590: F, t519: F, t806: F, t1446: F, t7605: F, t15582: F, t2158: F, t20861: F, t20864: F, t20868: F, t20870: F, t20873: F, t20876: F, t20879: F, t20882: F, t20885: F, t20886: F) -> (F, F, F, F) {
    let t20890 = F::new(4.0) / F::new(5.0) * t519 * t1440 * t15590 * t806;
    let t20892 = F::new(4.0) / F::new(5.0) * t1446 * t7605;
    let t20894 = F::new(4.0) / F::new(5.0) * t15582 * t2158;
    let t20895 = t20861 - t20864 + t20868 - t20870 - t20873 + t20876 + t20879 + t20882 + t20885 + t20886 + t20890 + t20892 - t20894;
    (t20890, t20892, t20894, t20895)
}
