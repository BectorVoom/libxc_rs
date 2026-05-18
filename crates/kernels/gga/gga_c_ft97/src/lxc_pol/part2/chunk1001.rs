//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 1001/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk1001<F: Float>(t15514: F, t4139: F, t2409: F, t4145: F, t2874: F, t2801: F, t992: F, t2882: F, t2881: F, t14075: F, t4265: F, t4267: F, t8392: F) -> (F, F, F, F, F) {
    let t15515 = t4139 * t15514;
    let t15518 = t4145 * t2409;
    let t15519 = t2874 * t15518;
    let t15522 = t992 * t2801;
    let t15523 = t2882 * t15522;
    let t15524 = t2881 * t15523;
    let t15527 = t4265 * t14075;
    let t15528 = t2881 * t15527;
    let t15532 = F::new(4.0) / F::new(27.0) * t8392 * t4267;
    (t15515, t15519, t15524, t15528, t15532)
}
