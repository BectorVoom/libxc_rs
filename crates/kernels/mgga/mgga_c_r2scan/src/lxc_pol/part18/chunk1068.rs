//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1068/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1068<F: Float>(t3602: F, t37755: F, t7619: F, t40033: F, t7624: F, t3606: F, t8081: F, t39935: F, t1055: F, t9085: F, t30281: F, t3332: F, t7628: F, t11717: F, t26278: F, t10760: F, t29700: F, t6085: F) -> (F, F, F, F, F, F, F, F) {
    let t43462 = t37755 * t3602 * t7619;
    let t43465 = t40033 * t3602 * t7624;
    let t43468 = t37755 * t3606 * t8081;
    let t43471 = t39935 * t3606 * t7619;
    let t43474 = t9085 * t1055;
    let t43477 = t7628 * t3332 * t30281;
    let t43480 = t26278 * t11717;
    let t43483 = t6085 * t10760 * t29700;
    (t43462, t43465, t43468, t43471, t43474, t43477, t43480, t43483)
}
