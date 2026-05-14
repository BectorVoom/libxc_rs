//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1163/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1163<F: Float>(t1377: F, t2676: F, t97: F, t2002: F, t5365: F, t5350: F, t4754: F, t815: F, t1874: F, t1887: F, t4682: F, t802: F, t17527: F, t17530: F, t17532: F, t17534: F, t17537: F, t17542: F, t17544: F, t17547: F) -> (F, F, F, F, F, F) {
    let t17550 = t2676 * t97 * t1377;
    let t17553 = 4.0 / 45.0 * t2002 * t5365;
    let t17555 = 4.0 / 15.0 * t2002 * t5350;
    let t17557 = t4754 * t815 / 15.0;
    let t17559 = 2.0 / 15.0 * t1887 * t1874;
    let t17561 = t802 * t4682 / 15.0;
    let t17562 = t17527 + t17530 - t17532 + t17534 - t17537 - t17542 + 0.21642082724729686 * t17544 + 0.21642082724729686 * t17547 + 0.011181742741110338 * t17550 + t17553 + t17555 + t17557 + t17559 + t17561;
    (t17553, t17555, t17557, t17559, t17561, t17562)
}
