//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1017/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1017<F: Float>(t51: F, t11007: F, t11030: F, t1719: F, t2719: F, t425: F, t2724: F, t6403: F, t2723: F, t4878: F, t6360: F, t1701: F, t2140: F, zeta_threshold: F) -> (F, F, F, F) {
    let t52 = t51 <= zeta_threshold;
    let t11033 = piecewise3::<F>(t52, t11007, t11030 * t425 + t1719 * t2719);
    let t11039 = F::cast_from(1.28_f64) * t6403 * t2724;
    let t11040 = t2723 * t4878;
    let t11042 = F::cast_from(1.28_f64) * t6360 * t11040;
    let t11045 = t1701 * t2140;
    (t11033, t11039, t11042, t11045)
}
