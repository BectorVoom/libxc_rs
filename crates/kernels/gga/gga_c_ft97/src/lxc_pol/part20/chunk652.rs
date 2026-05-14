//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 652/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk652<F: Float>(t2608: F, t3690: F, t14187: F, t1091: F, t2579: F, t10007: F, t2492: F, t265: F) -> (F, F, F, F, F) {
    let t14188 = t3690 * t2608;
    let t14189 = t14187 * t14188;
    let t14192 = t1091 * t2579;
    let t14193 = t10007 * t14192;
    let t14196 = t2492 * t265;
    (t14188, t14189, t14192, t14193, t14196)
}
