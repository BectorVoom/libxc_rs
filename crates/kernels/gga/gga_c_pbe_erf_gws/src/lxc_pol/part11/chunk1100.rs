//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1100/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1100<F: Float>(t2210: F, t49955: F, t858: F, t884: F, t19561: F, t21621: F, t21623: F, t49063: F, t860: F, t20137: F, t2300: F, t3247: F, t3808: F, t38506: F, t45852: F, t49064: F, t49092: F, t49842: F, t49950: F, t49952: F, t49954: F, t902: F, t904: F, t905: F, t929: F, t9665: F) -> (F, F, F) {
    let t49963 = 3.0 / 16.0 * t884 * t2210 * t858 * t49955;
    let t49980 = t21621 * t49063 * t19561 * t21623 * t860 / 96.0;
    let t49981 = t49950 + t49952 - 119.0 / 1152.0 * t38506 + t49954 + 5.0 / 256.0 * t929 * t2300 * t904 * t49955 + t49963 - 7.0 / 576.0 * t45852 + t902 * t905 * t3808 * t49092 / 256.0 - 3.0 / 32.0 * t3247 * t9665 * t49842 + t902 * t905 * t49064 * t20137 / 192.0 + t49980;
    (t49963, t49980, t49981)
}
