//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1036/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1036<F: Float>(t188: F, t3023: F, t398: F, t4641: F, t4913: F, t83: F, t1166: F, t1409: F, t1366: F, t3315: F, t3322: F, t27: F, t3018: F, t545: F) -> (F, F, F, F, F, F) {
    let t10720 = t398 * t3023 * t188;
    let t10727 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t83 * (-F::cast_from(4.277777777777778_f64) * t4641 + F::cast_from(220.0_f64) / F::cast_from(81.0_f64) * t4913) * t188;
    let t10735 = t1166 * t1409 * t188;
    let t10743 = t3315 * t1366;
    let t10746 = F::cast_from(0.4328416544945937_f64) * t3322 * t1366;
    let t10751 = t3018 * t27 * t545;
    (t10720, t10727, t10735, t10743, t10746, t10751)
}
