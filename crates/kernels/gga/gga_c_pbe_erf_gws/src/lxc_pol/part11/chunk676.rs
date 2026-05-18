//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 676/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk676<F: Float>(t2515: F, t414: F, t1336: F, t960: F, t1396: F, t2840: F, t1392: F, t1218: F, t242: F, t3013: F, t2519: F, t700: F) -> (F, F, F, F, F, F, F) {
    let t8012 = t414 * t2515;
    let t8014 = t1336 * t960;
    let t8016 = t2840 * t1396;
    let t8018 = t2840 * t1392;
    let t8023 = t2840 * t1218;
    let t8042 = t3013 * t242;
    let t8051 = t2519 * t700;
    (t8012, t8014, t8016, t8018, t8023, t8042, t8051)
}
