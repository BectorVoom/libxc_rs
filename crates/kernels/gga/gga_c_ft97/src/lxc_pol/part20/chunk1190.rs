//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1190/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1190<F: Float>(t1701: F, t2719: F, t27494: F, t109314: F, t28552: F, t24330: F, t25049: F, t28616: F, t109117: F, t6256: F, t14810: F, t6027: F, t2691: F, t28557: F, t111837: F, t4113: F) -> (F, F, F, F, F, F, F) {
    let t112127 = t1701 * t27494 * t2719;
    let t112133 = t28552 * t109314;
    let t112137 = 0.13335600218518518519e0 * t25049 * t24330 * t28616;
    let t112138 = t6256 * t109117;
    let t112153 = t1701 * t6027 * t14810;
    let t112156 = t2691 * t28557;
    let t112159 = t4113 * t111837;
    (t112127, t112133, t112137, t112138, t112153, t112156, t112159)
}
