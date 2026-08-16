//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2625/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2625<F: Float>(t40281: F, t5303: F, t12211: F, t16300: F, t5247: F, t820: F, t12250: F, t1824: F, t16288: F, t3853: F, t12384: F, t5234: F) -> (F, F, F, F, F, F) {
    let t53997 = t40281 * t5303;
    let t54003 = t12211 * t16300;
    let t54013 = t5247 * t820;
    let t54014 = t1824 * t12250;
    let t54034 = t16288 * t3853;
    let t54042 = t5234 * t12384;
    (t53997, t54003, t54013, t54014, t54034, t54042)
}
