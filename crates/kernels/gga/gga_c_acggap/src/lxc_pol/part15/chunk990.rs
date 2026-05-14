//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 990/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk990<F: Float>(t1181: F, t2068: F, t25732: F, t604: F, t142: F, t6379: F, t8806: F, t6383: F, t1318: F, t507: F, t7436: F, t6388: F, t5906: F, t5693: F, t8463: F, t5697: F, t7351: F, t7575: F) -> (F, F, F, F, F, F, F, F) {
    let t39462 = t2068 * t1181 * t604 * t25732;
    let t39465 = t8806 * t142 * t6379;
    let t39468 = t8806 * t142 * t6383;
    let t39471 = t7436 * t507 * t1318;
    let t39474 = t8806 * t142 * t6388;
    let t39477 = t7436 * t142 * t5906;
    let t39485 = t8463 * t1181 * t604 * t5693;
    let t39489 = t7575 * t1181 * t7351 * t5697;
    (t39462, t39465, t39468, t39471, t39474, t39477, t39485, t39489)
}
