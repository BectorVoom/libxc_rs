//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1238/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1238<F: Float>(t13088: F, t20440: F, t20442: F, t20445: F, t20449: F, t20451: F, t20452: F, t20454: F, t20456: F, t20460: F, t20463: F, t18274: F, t18277: F, t20465: F, t20467: F, t20472: F, t20478: F, t20480: F, t20482: F, t20486: F, t20490: F, t20491: F, t20492: F) -> (F, F) {
    let t21995 = -t20440 + t20442 + t20445 + t20449 + t20451 + t13088 - t20452 - t20454 - t20456 - t20460 + t20463;
    let t21997 = -t20465 - t20467 - t20472 + t20478 - t20480 + t20482 - t20486 + t20490 - t20491 - t20492 + t18274 + F::cast_from(0.18233333333333332_f64) * t18277;
    (t21995, t21997)
}
