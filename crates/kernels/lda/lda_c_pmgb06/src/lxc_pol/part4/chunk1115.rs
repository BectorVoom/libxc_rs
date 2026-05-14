//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1115/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1115<F: Float>(t15353: F, t1897: F, t439: F, t15358: F, t1901: F, t1420: F, t6419: F, t5253: F, t6165: F, t15363: F, t15367: F, t15411: F, t5260: F, t16649: F, t16652: F, t16654: F, t16657: F, t16659: F, t16662: F, t16665: F) -> (F, F, F, F, F, F, F, F) {
    let t16668 = 2.0 / 45.0 * t439 * t1897 * t15353;
    let t16671 = 2.0 / 9.0 * t439 * t1901 * t15358;
    let t16673 = 2.0 / 27.0 * t1420 * t6419;
    let t16676 = 2.0 / 27.0 * t439 * t5253 * t6165;
    let t16679 = 2.0 / 27.0 * t439 * t1901 * t15363;
    let t16682 = t439 * t1901 * t15367 / 27.0;
    let t16685 = 8.0 / 81.0 * t439 * t5260 * t15411;
    let t16686 = t16649 + t16652 + t16654 + t16657 - t16659 - t16662 - t16665 - t16668 - t16671 + t16673 + t16676 + t16679 + t16682 + t16685;
    (t16668, t16671, t16673, t16676, t16679, t16682, t16685, t16686)
}
