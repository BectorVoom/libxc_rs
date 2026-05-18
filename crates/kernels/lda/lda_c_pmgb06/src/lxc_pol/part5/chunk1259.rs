//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1259/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1259<F: Float>(t10492: F, t10494: F, t18407: F, t19131: F, t199: F, t22077: F, t22082: F, t22084: F, t22088: F, t399: F, t566: F, t6928: F, t7375: F, t7874: F, t84: F, t868: F) -> F {
    let t22097 = -F::new(0.5694518669548363) * t22077 + F::new(3.9861630686838536) * t18407 + F::new(0.5025769232130264) * t10492 + F::new(0.5025769232130264) * t10494 + F::new(0.2512884616065132) * t22082 + F::new(0.0837628205355044) * t22084 - F::new(0.0837628205355044) * t399 * t7375 - F::new(0.0837628205355044) * t84 * t22088 - F::new(0.0837628205355044) * t19131 * t199 - F::new(0.0837628205355044) * t7874 * t566 - F::new(0.2512884616065132) * t6928 * t868;
    t22097
}
