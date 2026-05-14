//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1055/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1055<F: Float>(t11868: F, t1385: F, t1868: F, t2010: F, t2064: F, t5168: F, t6372: F, t10412: F, t2500: F, t439: F, t1423: F, t6376: F, t15734: F, t15736: F, t15738: F, t15740: F, t15743: F, t15745: F, t15746: F, t15747: F, t15748: F, t15753: F) -> (F, F, F, F, F, F) {
    let t15754 = 2.0 / 45.0 * t11868;
    let t15758 = 8.0 / 45.0 * t2010 * t1385 * t1868 * t2064;
    let t15760 = 8.0 / 45.0 * t5168 * t6372;
    let t15763 = 2.0 / 45.0 * t439 * t10412 * t2500;
    let t15764 = t1423 * t6376;
    let t15765 = 8.0 / 135.0 * t15764;
    let t15766 = t15734 + t15736 - t15738 + t15740 - t15743 - t15745 - t15746 + t15747 - t15748 - t15753 + t15754 - t15758 - t15760 - t15763 + t15765;
    (t15754, t15758, t15760, t15763, t15765, t15766)
}
