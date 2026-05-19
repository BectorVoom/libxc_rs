//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 424/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk424<F: Float>(t2086: F, t254: F, t39: F, t9: F, t120: F, t122: F, t135: F, t57: F, t269: F) -> (F, F, F, F) {
    let t2088 = F::cast_from(0.42377972951376424087e0_f64) * t254 * t2086;
    let t2090 = F::new(1.0) / t9 / t39;
    let t2095 = F::cast_from(0.21341733463216935736e0_f64) * t120 * t122 * t2090 * t57 * t135;
    let t2096 = t269 * t269;
    (t2088, t2090, t2095, t2096)
}
