//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 162/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk162<F: Float>(t1147: F, t737: F, t1131: F, t743: F, t192: F, t462: F, t736: F, t92: F) -> (F, F, F) {
    let t1148 = t737 * t1147;
    let t1151 = t743 * t1131;
    let t1152 = t192 * t1151;
    let t1154 = -t736 - t462 * t1148 / F::cast_from(3.0_f64) - t92 * t1152;
    (t1148, t1152, t1154)
}
