//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1295/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1295<F: Float>(t56986: F, t57007: F, t57052: F, t57108: F, t779: F, t799: F, t4818: F, t24302: F, t24305: F, t16817: F, t3793: F, t845: F) -> (F, F, F, F) {
    let t57113 = F::cast_from(1.0_f64) * t779 * (t56986 + t57007 + t57052 + t57108) * t799;
    let t57114 = t4818 * t4818;
    let t57117 = F::cast_from(0.24954977986735470917e5_f64) * t24302 * t57114 * t24305;
    let t57120 = F::cast_from(0.46785787179641632568e1_f64) * t845 * t3793 * t16817;
    (t57113, t57114, t57117, t57120)
}
