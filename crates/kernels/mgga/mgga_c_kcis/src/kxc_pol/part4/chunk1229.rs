//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1229/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1229<F: Float>(t17428: F, t17431: F, t17434: F, t17437: F, t17439: F, t17441: F, t17444: F, t17447: F, t17451: F, t17455: F, t17458: F, t17461: F, t17465: F, t17468: F, t17472: F, t17475: F, t17478: F, t17481: F) -> (F,) {
    let t18331 = 0.23981481481481481481e-1 * t17428 - 0.125e0 * t17431 + 0.71944444444444444444e-1 * t17434 - 0.26979166666666666666e-1 * t17437 + 0.20234375e-1 * t17439 - 0.26979166666666666666e-1 * t17441 - 0.89930555555555555554e-2 * t17444 + 0.26979166666666666666e-1 * t17447 - 0.1875e0 * t17451 + 0.89930555555555555554e-2 * t17455 + 0.13489583333333333333e-1 * t17458 - 0.625e-1 * t17461 + 0.60703125e-1 * t17465 + 0.13489583333333333333e-1 * t17468 + 0.29976851851851851851e-2 * t17472 + 0.33333333333333333334e0 * t17475 + 0.1875e0 * t17478 + 0.25e0 * t17481;
    (t18331,)
}
