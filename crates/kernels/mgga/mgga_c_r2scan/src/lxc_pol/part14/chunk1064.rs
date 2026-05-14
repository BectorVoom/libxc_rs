//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1064/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1064<F: Float>(t3262: F, t3574: F, t38739: F, t3472: F, t40397: F, t3579: F, t39032: F, t12042: F, t37271: F, t3465: F, t39327: F, t38771: F, t10610: F, t10611: F, t12056: F, t10940: F, t12086: F) -> (F, F, F, F, F, F, F, F) {
    let t41280 = 3.0 / 4.0 * t3262 * t38739 * t3574;
    let t41283 = 15.0 / 16.0 * t3262 * t3472 * t40397;
    let t41285 = t3579 * t39032 / 2.0;
    let t41286 = t37271 * t12042;
    let t41289 = 3.0 / 4.0 * t3262 * t3465 * t39327;
    let t41291 = 5.0 / 8.0 * t3579 * t38771;
    let t41294 = 3.0 / 2.0 * t10610 * t12056 * t10611;
    let t41296 = t10940 * t12086 / 4.0;
    (t41280, t41283, t41285, t41286, t41289, t41291, t41294, t41296)
}
