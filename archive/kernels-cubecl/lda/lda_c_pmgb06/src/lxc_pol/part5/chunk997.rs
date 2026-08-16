//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 997/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk997<F: Float>(t2489: F, t3226: F, t1447: F, t6292: F, t441: F, t6673: F, t224: F, t6687: F, t118: F, t5988: F, t2414: F, t740: F) -> (F, F, F, F, F, F) {
    let t18008 = t3226 * t2489;
    let t18010 = t1447 * t6292;
    let t18016 = t441 * t6673;
    let t18020 = t6687 * t224;
    let t18054 = t5988 * t118;
    let t18057 = t740 * t2414;
    (t18008, t18010, t18016, t18020, t18054, t18057)
}
