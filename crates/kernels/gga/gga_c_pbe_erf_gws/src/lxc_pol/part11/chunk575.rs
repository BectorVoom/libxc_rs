//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 575/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk575<F: Float>(t4541: F, t700: F, t762: F, t1383: F, t528: F, t532: F, t4358: F, t35: F, t413: F, t1477: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t4542 = 6.0 * t4541;
    let t4550 = 0.50257692321302641125e0 * t762 * t700;
    let t4557 = 0.25128846160651320563e0 * t528 * t1383;
    let t4558 = 12.0 * t532;
    let t4559 = 36.0 * t4358;
    let t4560 = t35 * t413;
    let t4561 = 24.0 * t4560;
    let t4573 = t6 * t1477;
    (t4542, t4550, t4557, t4558, t4559, t4560, t4561, t4573)
}
