//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1081/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1081<F: Float>(t13061: F, t1990: F, t13110: F, t544: F, t1797: F, t4: F, t4579: F, t4665: F, t7061: F, t4715: F, t7022: F, t4712: F) -> (F, F, F, F, F, F) {
    let t38346 = t13061 * t1990;
    let t38368 = t544 * t13110;
    let t38375 = t4579 * t4 * t1797;
    let t38433 = t7061 * t4665;
    let t38444 = t7022 * t4715;
    let t38446 = t7022 * t4712;
    (t38346, t38368, t38375, t38433, t38444, t38446)
}
