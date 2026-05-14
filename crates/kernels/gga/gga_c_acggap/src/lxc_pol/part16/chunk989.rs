//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 989/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk989<F: Float>(t7413: F, t8480: F, t8947: F, t1181: F, t2068: F, t26108: F, t604: F, t25732: F, t142: F, t6379: F, t8806: F, t6383: F, t1318: F, t507: F, t7436: F, t6388: F) -> (F, F, F, F, F, F, F) {
    let t39454 = t7413 * t8480 * t8947;
    let t39458 = t2068 * t1181 * t604 * t26108;
    let t39462 = t2068 * t1181 * t604 * t25732;
    let t39465 = t8806 * t142 * t6379;
    let t39468 = t8806 * t142 * t6383;
    let t39471 = t7436 * t507 * t1318;
    let t39474 = t8806 * t142 * t6388;
    (t39454, t39458, t39462, t39465, t39468, t39471, t39474)
}
