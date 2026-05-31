//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1263/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1263<F: Float>(t12024: F, t37271: F, t12045: F, t37282: F, t11342: F, t40681: F, t12051: F, t1554: F, t3579: F, t15059: F, t795: F, t3270: F) -> (F, F, F, F, F) {
    let t42253 = F::cast_from(45.0_f64) / F::cast_from(32.0_f64) * t37271 * t12024;
    let t42255 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t37282 * t12045;
    let t42257 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t40681 * t11342;
    let t42260 = t3579 * t1554 * t12051 / F::cast_from(4.0_f64);
    let t42262 = t15059 * t795;
    let t42263 = t3270 * t42262;
    (t42253, t42255, t42257, t42260, t42263)
}
