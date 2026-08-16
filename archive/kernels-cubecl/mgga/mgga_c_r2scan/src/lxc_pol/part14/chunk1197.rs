//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1197/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1197<F: Float>(t3262: F, t3574: F, t38739: F, t3472: F, t40397: F, t3579: F, t39032: F, t12042: F, t37271: F, t3465: F, t39327: F, t38771: F) -> (F, F, F, F, F, F) {
    let t41280 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t38739 * t3574;
    let t41283 = F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t3262 * t3472 * t40397;
    let t41285 = t3579 * t39032 / F::cast_from(2.0_f64);
    let t41286 = t37271 * t12042;
    let t41289 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t3465 * t39327;
    let t41291 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3579 * t38771;
    (t41280, t41283, t41285, t41286, t41289, t41291)
}
