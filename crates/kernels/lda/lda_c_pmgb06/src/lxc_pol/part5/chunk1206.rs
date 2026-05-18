//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1206/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1206<F: Float>(t11142: F, t11147: F, t11150: F, t11152: F, t11156: F, t11157: F, t11161: F, t11162: F, t11165: F, t15026: F, t15028: F, t15030: F, t8814: F, t8822: F, t8830: F, t8834: F) -> F {
    let t21812 = F::new(10.526802520742363) * t11142 - F::new(155.84273195113317) * t11147 + t11150 + t11152 - F::new(12.0) * t15026 - F::new(12.0) * t15028 - F::new(24.0) * t15030 + t8814 + t8822 + t8830 - t8834 + t11156 - F::new(0.0017090684152272775) * t11157 + t11161 + F::new(311.68546390226635) * t11162 - t11165;
    t21812
}
