//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 39/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk39<F: Float>(t73: F, t105: F, t107: F, t108: F) -> (F, F) {
    let t112 = t73 * t73;
    let t114 = F::cast_from(0.19711288999999999999e-2_f64) * t105 * t107 * t108 - F::new(2.0) * t112;
    let t115 = F::new(1.0) / t114;
    (t114, t115)
}
