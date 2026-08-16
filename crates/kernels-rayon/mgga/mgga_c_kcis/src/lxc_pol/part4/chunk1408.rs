//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1408/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1408(t17428: f64, t17431: f64, t17434: f64, t17437: f64, t17439: f64, t17441: f64, t17444: f64, t17447: f64, t17451: f64, t17455: f64, t17458: f64, t17461: f64, t17465: f64, t17468: f64, t17472: f64, t17475: f64, t17478: f64, t17481: f64) -> f64 {
    let t18331 = 0.23981481481481481481e-1_f64 * t17428 - 0.125e0_f64 * t17431 + 0.71944444444444444444e-1_f64 * t17434 - 0.26979166666666666666e-1_f64 * t17437 + 0.20234375e-1_f64 * t17439 - 0.26979166666666666666e-1_f64 * t17441 - 0.89930555555555555554e-2_f64 * t17444 + 0.26979166666666666666e-1_f64 * t17447 - 0.1875e0_f64 * t17451 + 0.89930555555555555554e-2_f64 * t17455 + 0.13489583333333333333e-1_f64 * t17458 - 0.625e-1_f64 * t17461 + 0.60703125e-1_f64 * t17465 + 0.13489583333333333333e-1_f64 * t17468 + 0.29976851851851851851e-2_f64 * t17472 + 0.33333333333333333334e0_f64 * t17475 + 0.1875e0_f64 * t17478 + 0.25e0_f64 * t17481;
    t18331
}
