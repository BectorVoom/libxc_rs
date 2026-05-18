//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 959/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk959<F: Float>(t1882: F, t21686: F, t21515: F, t21510: F, t8392: F, t21541: F, t21549: F, t21682: F, t1095: F, t13411: F, t17818: F, t17836: F, t17868: F, t6757: F) -> (F, F, F, F, F, F, F, F) {
    let t79047 = t1882 * t21686;
    let t79138 = t1882 * t21515;
    let t79157 = t8392 * t21510;
    let t79179 = t1882 * t21541;
    let t79182 = t1882 * t21549;
    let t79218 = t1882 * t21682;
    let t79252 = t13411 * t1095;
    let t79253 = t79252 * t17818;
    let t79305 = t17836 * t17868 * t6757;
    (t79047, t79138, t79157, t79179, t79182, t79218, t79253, t79305)
}
