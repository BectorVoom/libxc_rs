//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 810/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk810<F: Float>(t2349: F, t25037: F, t10409: F, t446: F, t1882: F, t6336: F, t6260: F, t668: F) -> (F, F, F, F) {
    let t25038 = t25037 * t2349;
    let t25039 = t10409 * t25038;
    let t25040 = t446 * t25039;
    let t25042 = t1882 * t6336;
    let t25044 = t6260 * t668;
    (t25039, t25040, t25042, t25044)
}
