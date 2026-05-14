//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1010/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1010<F: Float>(t1882: F, t24457: F, t42050: F, t91: F, t24448: F, t24450: F, t681: F, t2404: F, t2506: F, t2347: F, t6061: F, t2360: F, t24540: F, t24533: F, t24434: F, t24543: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97156 = t1882 * t24457;
    let t97168 = t91 * t42050;
    let t97176 = t24448 * t681 * t24450;
    let t97181 = t2404 * t2506;
    let t97190 = t6061 * t2347;
    let t97198 = t6061 * t2360;
    let t97207 = t1882 * t24540;
    let t97209 = t1882 * t24533;
    let t97214 = t24543 * t24434;
    (t97156, t97168, t97176, t97181, t97190, t97198, t97207, t97209, t97214)
}
