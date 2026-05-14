//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1306/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1306<F: Float>(t11053: F, t11055: F, t11057: F, t11060: F, t11063: F, t11065: F, t11069: F, t11073: F, t11074: F, t11079: F, t17053: F, t17055: F, t17057: F, t17059: F, t17061: F, t17064: F, t17066: F) -> (F,) {
    let t19214 = t17053 + t17055 - t17057 + t17059 + t17061 + t17064 - t17066 + t11053 / 3.0 + 0.12155555555555556 * t11055 + 4.0 / 3.0 * t11057 + 4e-21 * t11060 + t11063 + 0.003030876351851852 * t11065 + t11069 + t11073 + 0.36466666666666664 * t11074 + t11079;
    (t19214,)
}
