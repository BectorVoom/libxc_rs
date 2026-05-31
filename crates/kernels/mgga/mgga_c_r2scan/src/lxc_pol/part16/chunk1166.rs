//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1166/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1166<F: Float>(t31393: F, t795: F, t3263: F, t3275: F, t113: F, t40393: F, t97: F, t11510: F, t11487: F, t40282: F, t3579: F, t40473: F) -> (F, F, F, F) {
    let t42940 = t31393 * t795;
    let t42943 = t3275 * t3263 * t42940 / F::cast_from(2.0_f64);
    let t42945 = t97 * t40393 * t113;
    let t42947 = F::cast_from(3.0_f64) * t42945 * t11510;
    let t42949 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t40282 * t11487;
    let t42951 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3579 * t40473;
    (t42943, t42947, t42949, t42951)
}
