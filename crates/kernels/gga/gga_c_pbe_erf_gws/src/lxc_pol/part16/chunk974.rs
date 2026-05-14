//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 974/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk974<F: Float>(t14001: F, t3960: F, t2087: F, t4023: F, t3969: F, t915: F, t2276: F) -> (F, F, F, F, F) {
    let t14002 = t14001 * t3960;
    let t14003 = 7.0 / 72.0 * t14002;
    let t14004 = t2087 * t4023;
    let t14006 = t3969 * t915;
    let t14007 = t2276 * t14006;
    (t14002, t14003, t14004, t14006, t14007)
}
