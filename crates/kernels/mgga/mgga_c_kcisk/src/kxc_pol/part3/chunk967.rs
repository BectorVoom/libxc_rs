//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 967/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk967<F: Float>(t13315: F, t13364: F, t13411: F, t13461: F, t14159: F, t14197: F, t14239: F, t14279: F, t504: F, t1458: F, t4163: F, t1520: F) -> (F, F) {
    let t14282 = t13315 + t13364 + t13411 + t13461 + t14159 + t14197 + t14239 + t14279;
    let t14283 = t14282 * t504;
    let t14284 = t4163 * t1458;
    let t14286 = F::new(3.0) * t14284 * t1520;
    (t14283, t14286)
}
