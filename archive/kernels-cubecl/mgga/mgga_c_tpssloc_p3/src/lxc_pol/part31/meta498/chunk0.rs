//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1693/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1693<F: Float>(t22845: F, t28073: F, t1998: F, t236: F, t6347: F, t6926: F, t6375: F, t6916: F, t22761: F, t6390: F, t2002: F, t6378: F) -> (F, F, F, F, F, F) {
    let t28074 = t22845 * t28073;
    let t28077 = t1998 * t236 * t6347;
    let t28078 = t6926 * t28077;
    let t28080 = t6916 * t6375;
    let t28085 = t22761 * t6390;
    let t28088 = t6378 * t2002;
    (t28074, t28077, t28078, t28080, t28085, t28088)
}
