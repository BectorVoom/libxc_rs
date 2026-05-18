//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1182/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1182<F: Float>(t1445: F, t46915: F, t574: F, t1: F, t106: F, t13749: F, t192: F, t536: F, t41790: F, t41793: F, t41794: F, t41798: F, t41800: F, t41813: F, t41814: F, t41818: F, t41820: F, t41829: F) -> F {
    let t47912 = F::new(0.46011511144704899612e1) * t574 * t1445 * t46915;
    let t47918 = t13749 * t1 * t106 * t192;
    let t47920 = F::new(0.35750489951850426669e0) * t536 * t47918;
    let t47921 = -t41790 + t41793 - F::new(0.46011511144704899612e1) * t41794 - F::new(0.46011511144704899612e1) * t41798 + t41800 - t47912 - t41813 - F::new(0.69017266717057349418e1) * t41814 - F::new(0.69017266717057349418e1) * t41818 + F::new(0.23833659967900284446e0) * t41820 - t41829 + t47920;
    t47921
}
