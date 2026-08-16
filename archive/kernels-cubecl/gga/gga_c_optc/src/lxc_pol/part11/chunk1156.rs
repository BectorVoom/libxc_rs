//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1156/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1156<F: Float>(t17185: F, t2367: F, t913: F, t10959: F, t17134: F, t2812: F, t17064: F, t930: F, t16988: F, t7433: F, t8127: F, t8129: F) -> (F, F, F, F, F) {
    let t51733 = t913 * t2367 * t17185;
    let t51736 = t2812 * t10959 * t17134;
    let t51743 = t930 * t2367 * t17064;
    let t51745 = t7433 * t16988;
    let t51747 = t8127 * t51745 * t8129;
    (t51733, t51736, t51743, t51745, t51747)
}
