//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 935/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk935<F: Float>(t2209: F, t2365: F, t6658: F, t825: F, t19562: F, t346: F, t6274: F, t6684: F, t6553: F, t899: F, t922: F, t6587: F, t912: F) -> (F, F, F, F, F, F) {
    let t20550 = t2365 * t2209;
    let t20560 = t825 * t6658;
    let t20585 = t19562 * t346;
    let t20607 = t6684 * t6274;
    let t20625 = t899 * t6553 * t922;
    let t20646 = t899 * t912 * t6587;
    (t20550, t20560, t20585, t20607, t20625, t20646)
}
