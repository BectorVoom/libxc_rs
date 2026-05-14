//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1428/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1428<F: Float>(t109494: F, t33905: F, t9536: F, t115002: F, t33781: F, t3936: F, t2331: F, t442: F, t32466: F, t33870: F, t9524: F, t2737: F, t33863: F, t4419: F, t32339: F, t33883: F) -> (F, F, F, F, F, F, F) {
    let t115661 = 0.11574074074074074074e-2 * t9536 * t109494 * t33905;
    let t115663 = 0.11574074074074074074e-2 * t9536 * t115002;
    let t115666 = t3936 * t33781;
    let t115667 = t2331 * t442;
    let t115669 = t115666 * t115667 * t32466;
    let t115676 = 0.34722222222222222222e-2 * t9524 * t33870;
    let t115679 = 0.34722222222222222222e-2 * t2737 * t4419 * t33863;
    let t115684 = t32339 * t33883;
    (t115661, t115663, t115667, t115669, t115676, t115679, t115684)
}
