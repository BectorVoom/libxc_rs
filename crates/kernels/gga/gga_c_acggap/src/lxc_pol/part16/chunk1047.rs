//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1047/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1047<F: Float>(t1846: F, t7685: F, t1817: F, t31811: F, t2030: F, t301: F, t8927: F, t9552: F, t2060: F, t36222: F, t372: F, t1524: F, t2288: F, t35413: F, t5697: F, t34903: F, t5693: F, t7450: F) -> (F, F, F, F, F, F, F) {
    let t40330 = t7685 * t1846;
    let t40332 = t31811 * t1817;
    let t40336 = t2030 * t8927 * t9552 * t301;
    let t40340 = t2060 * t36222 * t9552 * t372;
    let t40344 = t2060 * t8927 * t2288 * t1524;
    let t40347 = t2030 * t35413 * t5697;
    let t40350 = t7450 * t34903 * t5693;
    (t40330, t40332, t40336, t40340, t40344, t40347, t40350)
}
