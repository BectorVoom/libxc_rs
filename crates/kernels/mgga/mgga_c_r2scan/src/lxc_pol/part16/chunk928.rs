//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 928/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk928<F: Float>(t12703: F, t374: F, t1039: F, t3696: F, t2333: F, t3347: F, t1064: F, t6897: F, t3617: F, t2332: F, t6660: F, t815: F, t312: F, t320: F, t6659: F, t325: F, t326: F, t6691: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12704 = t12703 * t374;
    let t12705 = t1039 * t3696;
    let t12706 = 2.0 * t12705;
    let t13908 = t2333 * t3347;
    let t14160 = t6897 * t1064;
    let t14656 = t2333 * t3617;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0 / t19025;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    let t19203 = t325 / t6691 / t326;
    (t12704, t12706, t13908, t14160, t14656, t19026, t19146, t19155, t19203)
}
