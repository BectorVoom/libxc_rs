//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 874/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk874<F: Float>(t193: F, t35985: F, t89: F, t33978: F, t992: F, t2665: F, t446: F, t1212: F, t33983: F, t35972: F, t799: F, t27: F) -> (F, F, F, F, F, F, F) {
    let t35986 = t193 * t35985;
    let t35987 = t89 * t35986;
    let t35989 = t33978 * t992;
    let t35990 = t2665 * t35989;
    let t35991 = t446 * t35990;
    let t35993 = t33983 * t1212;
    let t35994 = t193 * t35993;
    let t35995 = t89 * t35994;
    let t35997 = t799 * t35972;
    let t35999 = t89 * t27 * t35997;
    (t35987, t35990, t35991, t35993, t35995, t35997, t35999)
}
