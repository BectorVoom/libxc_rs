//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 739/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk739<F: Float>(t5395: F, t8624: F, t5727: F, t5743: F, t5692: F) -> (F, F, F) {
    let t8704 = t5395 * t8624;
    let t8705 = t8704 * t5727;
    let t8707 = t8704 * t5743;
    let t8709 = F::cast_from(1.0_f64) / t5692;
    (t8705, t8707, t8709)
}
