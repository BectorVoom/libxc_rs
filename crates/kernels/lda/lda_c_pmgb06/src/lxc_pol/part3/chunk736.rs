//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 736/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk736<F: Float>(t1795: F, t566: F, t3: F, t4463: F, t1329: F, t868: F, t1808: F, t391: F, t1200: F, t1338: F, t1799: F, t199: F, t399: F, t4187: F, t4212: F, t4214: F, t4216: F, t4218: F, t4220: F, t4435: F, t795: F, t84: F) -> (F, F) {
    let t5542 = 0.1675256410710088 * t1795 * t566;
    let t5543 = t3 * t4463;
    let t5551 = 0.1675256410710088 * t1329 * t868;
    let t5553 = 0.1675256410710088 * t391 * t1808;
    let t5563 = -0.0837628205355044 * t84 * t4435 + t5542 - 0.0837628205355044 * t5543 * t199 - 0.1675256410710088 * t1799 * t566 - 0.0837628205355044 * t795 * t1200 + t5551 + t5553 - 0.0837628205355044 * t1338 * t868 - 0.1675256410710088 * t399 * t1808 + t4187 - 0.3350512821420176 * t4212 - 0.3350512821420176 * t4214 + 0.0837628205355044 * t4216 + 0.1675256410710088 * t4218 + 0.0837628205355044 * t4220;
    (t5543, t5563)
}
