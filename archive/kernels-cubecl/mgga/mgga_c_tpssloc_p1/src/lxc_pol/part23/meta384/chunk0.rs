//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1187/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1187<F: Float>(t11677: F, t15027: F, t3624: F, t52627: F, t1213: F, t1735: F, t248: F, t45017: F, t10477: F, t1742: F, t11713: F, t3503: F) -> (F, F, F, F, F) {
    let t52879 = t15027 * t11677;
    let t52903 = t3624 * t52627;
    let t53079 = t1213 * t248 * t45017 * t1735;
    let t53081 = t1742 * t10477;
    let t53083 = t11713 * t3503 * t53081;
    (t52879, t52903, t53079, t53081, t53083)
}
