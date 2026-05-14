//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1264/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1264<F: Float>(t10964: F, t10967: F, t10980: F, t10984: F, t10990: F, t14773: F, t14776: F, t14831: F, t14866: F, t15081: F, t15124: F, t18444: F, t18486: F, t18892: F, t19045: F, t19055: F, t19069: F, t2: F, t328: F, t8028: F) -> (F,) {
    let tv4rho42 = 0.9480012043054112 * t10967 - 2.530897186465939 * t10980 + 0.8215265768013333 * t14773 + 0.5476843845342222 * t10984 + 2.0 * t14776 + t2 * (t14831 + t14866 + t15081 + t15124 + t18444 + t18486 + t18892 + t19045) * t328 + 0.6327242966164848 * t10964 + 2.530897186465939 * t10990 + 0.13692109613355555 * t19055 + 0.13692109613355555 * t8028 + t19069;
    (tv4rho42,)
}
