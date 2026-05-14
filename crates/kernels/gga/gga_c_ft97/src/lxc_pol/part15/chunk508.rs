//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 508/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk508<F: Float>(t2843: F, t5309: F, t296: F, t1091: F, t1255: F, t835: F, t319: F, t4973: F, t2857: F, t4965: F, t2862: F, t5225: F, t1212: F, t1248: F) -> (F, F, F, F, F, F, F) {
    let t5310 = t2843 * t5309;
    let t5311 = t296 * t5310;
    let t5315 = t835 * t1255 * t1091;
    let t5319 = t835 * t319 * t4973;
    let t5323 = t2857 * t319 * t4965;
    let t5327 = t2862 * t319 * t5225;
    let t5330 = t1212 * t1248;
    (t5310, t5311, t5315, t5319, t5323, t5327, t5330)
}
