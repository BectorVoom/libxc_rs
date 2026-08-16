//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 876/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk876<F: Float>(t173: F, t7559: F, t184: F, t199: F, t626: F, t7483: F, t2735: F, t211: F, t1046: F, t1783: F, t2723: F, t582: F) -> (F, F, F, F) {
    let t7560 = t173 * t7559;
    let t7561 = t7560 * t184;
    let t7563 = F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t7561 * t199;
    let t7564 = t7483 * t626;
    let t7565 = t2735 * t7564;
    let t7567 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t211 * t7565;
    let t7569 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1783 * t1046;
    let t7570 = t582 * t2723;
    (t7563, t7567, t7569, t7570)
}
