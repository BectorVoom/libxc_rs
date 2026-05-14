//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1218/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1218<F: Float>(t16837: F, t16840: F, t16841: F, t16843: F, t16847: F, t16849: F, t16852: F, t16855: F, t16860: F, t16862: F, t16864: F, t16865: F, t16867: F, t16870: F, t16873: F, t208: F, t213: F, t579: F, t6716: F) -> (F, F) {
    let t18267 = -t16837 - t16840 - t16841 - t16843 + t16847 + t16849 + t16852 - t16855 - t16860 - t16862 - t16864 + t16865 - t16867 - t16870 + t16873;
    let t18274 = t6716 * t579 * t208 * t213;
    (t18267, t18274)
}
