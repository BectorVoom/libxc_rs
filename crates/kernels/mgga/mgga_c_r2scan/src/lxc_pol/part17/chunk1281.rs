//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1281/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1281<F: Float>(t11336: F, t40594: F, t42847: F, t39030: F, t40574: F, t43744: F, t3719: F, t910: F, t3270: F, t10667: F, t3262: F, t3465: F, t43959: F) -> (F, F, F, F) {
    let t45023 = F::new(45.0) / F::new(32.0) * t40594 * t11336 * t42847;
    let t45026 = F::new(5.0) / F::new(4.0) * t40574 * t39030 * t43744;
    let t45027 = t3719 * t910;
    let t45028 = t3270 * t45027;
    let t45030 = F::new(3.0) / F::new(2.0) * t10667 * t45028;
    let t45034 = F::new(3.0) / F::new(2.0) * t3262 * t3465 * t43959;
    (t45023, t45026, t45030, t45034)
}
