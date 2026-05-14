//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 645/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk645<F: Float>(t14116: F, t3885: F, t2606: F, t3892: F, t3891: F, t3871: F, t8392: F, t255: F, t676: F) -> (F, F, F, F, F, F) {
    let t14117 = t3885 * t14116;
    let t14118 = t2606 * t14117;
    let t14121 = t3892 * t14116;
    let t14122 = t3891 * t14121;
    let t14126 = 2.0 / 27.0 * t8392 * t3871;
    let t14127 = t676 * t255;
    (t14117, t14118, t14121, t14122, t14126, t14127)
}
