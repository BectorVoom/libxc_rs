//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 449/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk449<F: Float>(t627: F, t661: F, t1815: F, t639: F, t560: F, t586: F) -> (F, F, F, F) {
    let t1816 = t627 * t661;
    let t1817 = t1815 * t1816;
    let t1819 = 8.0 / 45.0 * t639 * t1817;
    let t1820 = t560 * t586;
    (t1816, t1817, t1819, t1820)
}
