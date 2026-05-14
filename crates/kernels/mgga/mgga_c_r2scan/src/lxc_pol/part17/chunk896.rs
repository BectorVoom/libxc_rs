//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 896/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk896<F: Float>(t12056: F, t3275: F, t3352: F, t11518: F, t3472: F, t3262: F, t1561: F, t3718: F) -> (F, F, F, F, F, F) {
    let t12092 = t3275 * t12056 * t3352;
    let t12093 = t12092 / 4.0;
    let t12094 = t3472 * t11518;
    let t12095 = t3262 * t12094;
    let t12096 = 15.0 / 16.0 * t12095;
    let t12098 = t1561 * t3718;
    (t12092, t12093, t12094, t12095, t12096, t12098)
}
