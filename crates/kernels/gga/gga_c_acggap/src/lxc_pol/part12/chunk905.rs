//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 905/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk905<F: Float>(t2068: F, t4680: F, t8907: F, t1181: F, t4540: F, t604: F, t7575: F, t5111: F, t4291: F, t7561: F, t4295: F, t7822: F, t4300: F, t4304: F, t30374: F, t8657: F) -> (F, F, F, F, F, F, F, F) {
    let t34138 = t2068 * t4680 * t8907;
    let t34142 = t7575 * t1181 * t604 * t4540;
    let t34146 = t7575 * t1181 * t604 * t5111;
    let t34148 = t7561 * t4291;
    let t34150 = t7822 * t4295;
    let t34152 = t7822 * t4300;
    let t34154 = t7822 * t4304;
    let t34156 = t30374 * t8657;
    (t34138, t34142, t34146, t34148, t34150, t34152, t34154, t34156)
}
