//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2975/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2975<F: Float>(t14077: F, t4630: F, t10401: F, t246: F, t3067: F, t3186: F, t1615: F, t3061: F, t375: F, t1022: F, t3961: F, t3200: F) -> (F, F, F, F, F, F) {
    let t62049 = t14077 * t4630;
    let t62053 = t10401 * t246;
    let t62054 = t3067 * t62053;
    let t62055 = t3186 * t62054;
    let t62057 = t375 * t3061 * t1615;
    let t62059 = t3961 * t1022;
    let t62064 = t3200 * t62054;
    (t62049, t62053, t62055, t62057, t62059, t62064)
}
