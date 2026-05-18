//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 843/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk843<F: Float>(t13086: F, t339: F, t1130: F, t11706: F, t13156: F, t13325: F, t13328: F, t2181: F, t3154: F, t340: F, t3848: F, t3851: F, t6429: F, t870: F, t9056: F) -> (F, F) {
    let t13331 = t339 * t13086;
    let t13334 = -t13156 * t339 * t340 + F::new(9.0) * t1130 * t11706 + F::new(60.0) * t13325 * t6429 - F::new(36.0) * t13328 * t2181 + F::new(3.0) * t13331 * t870 + F::new(9.0) * t3154 * t3851 - F::new(36.0) * t3848 * t9056;
    (t13331, t13334)
}
