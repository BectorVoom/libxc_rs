//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 923/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk923<F: Float>(t23483: F, t23553: F, t23951: F, t24055: F, t558: F, t614: F, t5778: F, t28: F, t165: F, t2075: F, t23925: F, t5779: F, t376: F, t5780: F, t1349: F, t160: F, t24046: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24057 = t23483 + t23553 + t23951 + t24055;
    let t24059 = t614 * t558;
    let t24060 = t5778 * t24059;
    let t24061 = t28 * t24060;
    let t24064 = t165 * t2075;
    let t24065 = t5778 * t24064;
    let t24066 = t28 * t24065;
    let t24069 = t23925 * t5779;
    let t24070 = t28 * t24069;
    let t24073 = t376 * t5780;
    let t24074 = t1349 * t24073;
    let t24078 = t24046 * t160;
    (t24057, t24059, t24060, t24061, t24064, t24065, t24066, t24069, t24070, t24073, t24074, t24078)
}
