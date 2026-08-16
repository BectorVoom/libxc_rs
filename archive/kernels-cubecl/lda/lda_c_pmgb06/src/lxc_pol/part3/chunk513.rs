//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 513/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk513<F: Float>(t107: F, t110: F, t122: F, t1658: F, t1659: F, t1661: F, t1672: F, t1674: F, t1741: F, t1743: F, t1796: F, t1799: F, t1804: F, t1808: F, t1813: F, t199: F, t202: F, t2116: F, t2122: F, t2164: F, t399: F, t566: F, t795: F, t84: F, t868: F) -> F {
    let t2168 = -t1658 + F::cast_from(0.0837628205355044_f64) * t1659 + F::cast_from(0.0837628205355044_f64) * t1661 + F::cast_from(0.0837628205355044_f64) * t1796 - F::cast_from(0.0837628205355044_f64) * t1799 * t199 - F::cast_from(0.0837628205355044_f64) * t795 * t566 + F::cast_from(0.0837628205355044_f64) * t1804 - F::cast_from(0.0837628205355044_f64) * t399 * t868 - F::cast_from(0.0837628205355044_f64) * t84 * t1808 - t1672 + F::cast_from(0.019897291109174608_f64) * t1674 + F::cast_from(0.019897291109174608_f64) * t1813 - F::cast_from(0.011938374665504766_f64) * t122 * t202 * t2116 + t1741 - F::cast_from(0.5694518669548363_f64) * t1743 - F::cast_from(0.5694518669548363_f64) * t2122 + F::cast_from(0.42708890021612717_f64) * t107 * t110 * t2164;
    t2168
}
