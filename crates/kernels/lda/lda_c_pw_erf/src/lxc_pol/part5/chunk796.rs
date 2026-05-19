//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 796/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk796<F: Float>(t125: F, t1550: F, t1808: F, t1881: F, t2592: F, t2645: F, t3203: F, t411: F, t4125: F, t4129: F, t4132: F, t4136: F, t4140: F, t4144: F, t456: F, t5933: F, t5941: F, t6097: F, t7083: F, t7085: F, t7214: F, t7231: F, t7302: F, t777: F) -> F {
    let t7305 = F::cast_from(0.19816831758676853_f64) * t3203 + t1881 * t2592 + t777 * t7083 + F::new(12.0) * t1808 * t7085 + t2645 * t1550 + F::new(6.0) * t1808 * t6097 * t411 + t7214 * t456 - F::cast_from(3.64371538634302e-05_f64) * t5933 - F::cast_from(0.0005811348303577384_f64) * t4125 - t4129 + F::cast_from(0.001355981270834723_f64) * t4132 + t4136 - t4140 - t4144 - t5941 + (t7231 + t7302) * t125;
    t7305
}
