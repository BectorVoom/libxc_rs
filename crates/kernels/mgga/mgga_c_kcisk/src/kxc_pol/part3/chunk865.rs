//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 865/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk865<F: Float>(t13315: F, t13364: F, t13411: F, t13461: F, t14159: F, t14197: F, t14239: F, t14279: F, t504: F, t1458: F, t4163: F, t1520: F, t1455: F, t4169: F, t4171: F, t4165: F, t4321: F) -> (F, F, F, F) {
    let t14282 = t13315 + t13364 + t13411 + t13461 + t14159 + t14197 + t14239 + t14279;
    let t14283 = t14282 * t504;
    let t14284 = t4163 * t1458;
    let t14286 = 3.0 * t14284 * t1520;
    let t14287 = t1455 * t4169;
    let t14289 = 6.0 * t14287 * t4171;
    let t14291 = 3.0 * t4165 * t4321;
    (t14283, t14286, t14289, t14291)
}
