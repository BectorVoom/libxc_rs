//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1380/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1380<F: Float>(t1339: F, t19770: F, t33608: F, t109717: F, t6234: F, t33363: F, t3748: F, t19734: F, t33367: F, t110025: F, t2232: F, t415: F, t2212: F, t32169: F, t3805: F, t9818: F) -> (F, F, F, F, F, F, F, F) {
    let t114444 = t1339 * t33608 * t19770;
    let t114448 = t1339 * t109717 * t6234;
    let t114453 = t3748 * t33363;
    let t114454 = 0.22109259259259259258e-2 * t114453;
    let t114455 = t19734 * t33367;
    let t114458 = t415 * t110025 * t2232;
    let t114462 = t415 * t2212 * t32169;
    let t114464 = t3805 * t9818;
    (t114444, t114448, t114453, t114454, t114455, t114458, t114462, t114464)
}
