//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1225/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1225<F: Float>(t13160: F, t4595: F, t16287: F, t3441: F, t4649: F, t6: F, t127: F, t2024: F, t4615: F, t4599: F, t141: F, t22836: F) -> (F, F, F, F, F, F, F, F) {
    let t56193 = t13160 * t4595;
    let t56197 = t3441 * t16287;
    let t56203 = t4649 * t4649;
    let t56204 = t6 * t56203;
    let t56205 = t56204 * t127;
    let t56209 = t56204 * t2024;
    let t56213 = t4615 * t4595;
    let t56222 = t4599 * t4599;
    let t56224 = t22836 * t141 * t56222;
    (t56193, t56197, t56203, t56205, t56209, t56213, t56222, t56224)
}
