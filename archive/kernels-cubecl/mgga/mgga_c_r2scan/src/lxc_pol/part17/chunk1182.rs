//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1182/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1182<F: Float>(t3602: F, t37755: F, t7619: F, t40033: F, t7624: F, t3606: F, t8081: F, t39935: F, t1055: F, t9085: F, t30281: F, t3332: F, t7628: F) -> (F, F, F, F, F, F) {
    let t43462 = t37755 * t3602 * t7619;
    let t43465 = t40033 * t3602 * t7624;
    let t43468 = t37755 * t3606 * t8081;
    let t43471 = t39935 * t3606 * t7619;
    let t43474 = t9085 * t1055;
    let t43477 = t7628 * t3332 * t30281;
    (t43462, t43465, t43468, t43471, t43474, t43477)
}
