//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1378/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1378<F: Float>(t107: F, t410: F, t6104: F, t122: F, t1669: F, t2659: F, t11744: F, t11747: F, t1200: F, t14231: F, t14233: F, t14235: F, t14237: F, t14240: F, t1799: F, t18066: F, t1808: F, t199: F, t2454: F, t5543: F, t566: F, t6928: F, t868: F) -> F {
    let t18141 = t107 * t410 * t6104;
    let t18144 = t122 * t1669 * t2659;
    let t18151 = -F::new(0.3350512821420176) * t1799 * t1808 + F::new(5.314884091578472) * t11744 - F::new(8.858140152630787) * t11747 - F::new(0.0837628205355044) * t18066 * t199 - F::new(0.1675256410710088) * t6928 * t566 - F::new(0.0837628205355044) * t2454 * t1200 - F::new(0.1675256410710088) * t5543 * t868 - F::new(1.1389037339096726) * t18141 - F::new(0.053059442957798957) * t18144 - F::new(0.6701025642840353) * t14231 - F::new(0.6701025642840353) * t14233 - F::new(0.6701025642840353) * t14235 - F::new(0.6701025642840353) * t14237 + F::new(0.1675256410710088) * t14240;
    t18151
}
