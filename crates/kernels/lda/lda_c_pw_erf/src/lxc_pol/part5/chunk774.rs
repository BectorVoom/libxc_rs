//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 774/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk774<F: Float>(t2599: F, t415: F, t325: F, t2611: F, t3313: F, t3322: F, t426: F, t5598: F, t5609: F, t7143: F, t7146: F, t7149: F, t7152: F, t7155: F) -> (F, F, F, F, F, F, F) {
    let t7158 = t415 * t2599;
    let t7159 = t7158 * t325;
    let t7160 = F::cast_from(0.9743416666666667_f64) * t7159;
    let t7161 = t415 * t2611;
    let t7162 = t7161 * t325;
    let t7163 = F::cast_from(0.48717083333333333_f64) * t7162;
    let t7164 = -t5598 - t5609 - t7143 / F::new(2.0) + t7146 / F::new(6.0) - F::new(2.93808) * t7149 + F::new(0.73452) * t7152 - t426 * t7155 / F::new(2.0) - t7160 + t7163 + t3313 - t3322;
    (t7158, t7159, t7160, t7161, t7162, t7163, t7164)
}
