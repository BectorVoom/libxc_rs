//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 682/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk682<F: Float>(t1378: F, t1971: F, t8361: F, t1049: F, t1986: F, t2007: F, t2970: F, t2000: F, t20: F, t2653: F, t2004: F, t678: F) -> (F, F, F, F, F, F, F) {
    let t8390 = t8361 * t1378 * t1971;
    let t8405 = t1049 * t1986;
    let t8408 = t2970 * t2007;
    let t8414 = t2970 * t2000;
    let t8424 = t2653 * t20;
    let t8425 = t8424 * t2004;
    let t8440 = t1049 * t678;
    (t8390, t8405, t8408, t8414, t8424, t8425, t8440)
}
