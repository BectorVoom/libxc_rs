//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1003/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1003<F: Float>(t3229: F, t797: F, t3275: F, t3276: F, t10945: F, t10948: F, t10957: F, t10965: F, t10970: F, t10974: F, t10983: F, t10991: F, t10996: F, t11616: F) -> (F, F, F) {
    let t12428 = t797 * t3229;
    let t12430 = t3275 * t3276 * t12428;
    let t12431 = F::new(5.0) / F::new(16.0) * t12430;
    let t12432 = t10945 + t10948 + t10957 - t10965 + t10970 + t10974 - t10983 - F::cast_from(0.81300399444200075504e-3_f64) * t11616 + t10991 + t10996 + t12431;
    (t12428, t12431, t12432)
}
