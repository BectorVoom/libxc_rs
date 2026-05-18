//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 467/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk467<F: Float>(t370: F, t7211: F, t27: F, t89: F, t7246: F, t7250: F, t7254: F, t7258: F) -> (F, F, F) {
    let t7260 = t370 * t7211;
    let t7262 = t89 * t27 * t7260;
    let t7264 = -t7246 / F::new(3.0) + t7250 / F::new(3.0) - t7254 / F::new(6.0) + F::new(2.0) / F::new(3.0) * t7258 - t7262 / F::new(3.0);
    (t7260, t7262, t7264)
}
