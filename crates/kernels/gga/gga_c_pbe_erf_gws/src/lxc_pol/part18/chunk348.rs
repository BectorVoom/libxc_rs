//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 348/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk348<F: Float>(t1008: F, t1012: F, t1021: F, t1026: F, t1035: F, t1039: F, t1048: F, t1049: F, t231: F, t585: F, t638: F, t674: F, t681: F, t683: F, t999: F, t247: F, t991: F) -> (F, F) {
    let t1052 = t999 + t1008 + t585 + t1012 - t1021 + t1026 + t1035 + t638 + t1039 - t1048 + 4.0 / 3.0 * t1049 * t231 + t674 + t681 + t683;
    let t1061 = t991 * t247;
    (t1052, t1061)
}
