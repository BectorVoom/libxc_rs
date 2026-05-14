//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1090/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1090<F: Float>(t1799: F, t1808: F, t18418: F, t18420: F, t18422: F, t18424: F, t18426: F, t18428: F, t18430: F, t18432: F, t18434: F, t2422: F, t2454: F, t6939: F, t795: F, t1322: F, t7353: F) -> (F, F) {
    let t22113 = -0.2512884616065132 * t2454 * t1808 - 0.2512884616065132 * t1799 * t2422 - 0.2512884616065132 * t795 * t6939 - 1.0051538464260528 * t18418 - 0.5025769232130264 * t18420 - 0.5025769232130264 * t18422 + 0.5025769232130264 * t18424 + 0.2512884616065132 * t18426 + 0.2512884616065132 * t18428 + 0.2512884616065132 * t18430 + 0.2512884616065132 * t18432 + 0.5025769232130264 * t18434;
    let t22120 = t7353 * t1322;
    (t22113, t22120)
}
