//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1363/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1363<F: Float>(t101: F, t10832: F, t142: F, t14503: F, t14515: F, t1556: F, t1568: F, t1808: F, t1809: F, t1859: F, t1880: F, t1881: F, t19425: F, t19432: F, t19449: F, t19702: F, t19794: F, t2591: F, t2592: F, t2645: F, t2765: F, t2775: F, t2778: F, t2791: F, t2805: F, t2810: F, t411: F, t4117: F, t4449: F, t454: F, t456: F, t5548: F, t5667: F, t5670: F, t5783: F, t5924: F, t6015: F, t6097: F, t6098: F, t6129: F, t6156: F, t7083: F, t7214: F, t777: F) -> (F,) {
    let t19808 = 24.0 * t5924 * t19425 - 6.0 * t5783 * t10832 * t6015 + t2645 * t2791 + 2.0 * t777 * t19432 * t2778 + 3.0 * t4117 * t6098 - 2.0 * t7214 * t1556 - 6.0 * t5783 * t2765 * t1859 * t411 + 4.0 * t101 * t1880 * t2775 * t6156 + 0.039914113367515366 * t14503 + 12.0 * t5924 * t19449 + 0.005423925083338892 * t14515 + 6.0 * t4449 * t2591 * t2810 + 2.0 * t1881 * t7083 + 6.0 * t1808 * t6097 * t1568 + t101 * (t19702 + t19794) * t456 + t777 * t454 * t5667 * t142 + 12.0 * t1808 * t1809 * t5548 + t5670 * t2592 - 2.0 * t777 * t2805 * t6129;
    (t19808,)
}
