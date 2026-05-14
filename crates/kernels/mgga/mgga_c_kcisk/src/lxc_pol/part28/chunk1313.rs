//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1313/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1313<F: Float>(t48363: F, t79: F, t1849: F, t5439: F, t33276: F, t9733: F, t25: F, t33218: F, t1310: F, t2020: F, t5005: F, t33257: F, t9739: F, t1774: F, t5507: F, t11966: F) -> (F, F, F, F, F, F, F, F) {
    let t112835 = t48363 * t79;
    let t112842 = t5439 * t1849;
    let t112856 = t9733 * t33276;
    let t112858 = t25 * t33218;
    let t112867 = t1310 * t5005 * t2020;
    let t112872 = t33257 * t9739;
    let t112876 = t1310 * t1774 * t5507;
    let t112904 = t2020 * t11966;
    (t112835, t112842, t112856, t112858, t112867, t112872, t112876, t112904)
}
