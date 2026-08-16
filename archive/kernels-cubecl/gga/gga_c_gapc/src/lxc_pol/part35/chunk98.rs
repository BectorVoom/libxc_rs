//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 98/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk98<F: Float>(t147: F, t19: F, t286: F, t145: F) -> (F, F) {
    let t293 = t286 * t19 * t147;
    let t296 = F::cast_from(30.0_f64) + t145 * t293 / F::cast_from(48.0_f64);
    let t297 = F::cast_from(1.0_f64) / t296;
    (t296, t297)
}
