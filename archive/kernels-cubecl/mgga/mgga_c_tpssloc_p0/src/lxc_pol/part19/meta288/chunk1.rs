//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1057/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1057<F: Float>(t12211: F, t3766: F, t119: F, t12156: F, t210: F, t1358: F, t3774: F, t1333: F, t3862: F, t10022: F, t248: F, t557: F) -> (F, F, F, F, F) {
    let t12317 = t12211 * t3766;
    let t12320 = t210 * t119 * t12156;
    let t12323 = t3774 * t1358;
    let t12325 = t1333 * t3862;
    let t12328 = t10022 * t557 * t248;
    (t12317, t12320, t12323, t12325, t12328)
}
