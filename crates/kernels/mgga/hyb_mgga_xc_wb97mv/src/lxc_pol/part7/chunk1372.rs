//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1372/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1372<F: Float>(t1128: F, t2869: F, t4554: F, t2873: F, t10166: F, t1127: F, t1132: F, t11734: F, t12054: F, t2860: F, t2900: F, t2915: F, t2946: F, t2957: F, t33497: F, t33516: F, t33519: F, t33540: F, t33544: F, t33551: F, t33901: F, t3729: F, t4591: F, t4610: F, t7918: F, t7938: F, t8089: F, t9989: F) -> (F,) {
    let t33924 = t1128 * t4554 * t2869;
    let t33928 = t1128 * t4554 * t2873;
    let t33951 = 0.35555555555555555555e0 * t10166 * t33901 + 120.0 * t7938 * t4591 * t2869 - 180.0 * t2860 * t11734 * t2873 - 0.108e0 * t2915 * t33924 + 0.126e0 * t2957 * t33928 - 0.32e-1 * t9989 * t4610 + 0.36e-1 * t8089 * t33540 - 0.72e-1 * t7918 * t33544 - 0.32e-1 * t3729 * t12054 - 0.256e-3 * t1127 * t33551 - 0.12e-1 * t2900 * t33924 + 0.18e-1 * t2946 * t33928 + 0.256e-3 * t1132 * t33516 - 0.256e-3 * t1127 * t33519 + 0.256e-3 * t1132 * t33497;
    (t33951,)
}
