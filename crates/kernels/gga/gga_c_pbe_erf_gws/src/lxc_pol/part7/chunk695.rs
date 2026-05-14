//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 695/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk695<F: Float>(t1371: F, t1952: F, t553: F, t1378: F, t1971: F, t5697: F, t1354: F, t331: F, t551: F, t1960: F, t5990: F, t5993: F, t5994: F, t5996: F, t5999: F, t6003: F, t6005: F, t6008: F) -> (F, F) {
    let t6012 = 0.19753890328909480882e-1 * t1952 * t1371 * t553;
    let t6015 = 0.34679929861433484636e-2 * t5697 * t1378 * t1971;
    let t6016 = t331 * t1354;
    let t6018 = t6016 * t551 * t553;
    let t6021 = t1960 * t1371 * t553;
    let t6023 = -0.18903244333884670701e0 * t5990 - t5993 + 0.94516221669423353502e-1 * t5994 + 0.18903244333884670701e0 * t5996 + t5999 + t6003 - t6005 + 0.19753890328909480882e-1 * t6008 + t6012 + t6015 - 0.59261670986728442646e-2 * t6018 - 0.11852334197345688529e-1 * t6021;
    (t6016, t6023)
}
