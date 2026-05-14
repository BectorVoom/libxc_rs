//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1367/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1367<F: Float>(t1: F, t1750: F, t1755: F, t2686: F, t15288: F, t15478: F, t18837: F, t18869: F, t18899: F, t18966: F, t18967: F, t18968: F, t18969: F, t18971: F, t18972: F, t18974: F, t18975: F, t18977: F, t18982: F, t19041: F, t19424: F, t19808: F, t19828: F, t19875: F, t312: F, t8423: F, t8427: F, t8432: F, t8437: F, t8445: F, t8449: F) -> (F,) {
    let t19882 = t2686 * t1750 * t1 * t1755;
    let t19885 = t15478 - (t18837 + t18869 + t18899 + t19041 + t19424 + t19808 + t19828 + t19875) * t312 + t18966 - t18967 + t18968 + t18969 + t8423 - t8427 - 0.6327242966164848 * t19882 + t18971 + t8432 + t8437 - t18972 + t8445 - t8449 + t18974 - t18975 + t18977 - 0.8215265768013333 * t15288 - t18982;
    (t19885,)
}
