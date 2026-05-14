//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 830/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk830<F: Float>(t1683: F, t1750: F, t1820: F, t1885: F, t5273: F, t562: F, t597: F, t16978: F, t639: F, t642: F, t643: F, t1627: F, t5464: F, t4998: F, t1631: F, t5467: F) -> (F, F, F, F, F, F) {
    let t17057 = t1750 * t1683;
    let t17058 = 16.0 / 15.0 * t17057;
    let t17063 = 16.0 / 15.0 * t1820 * t1885 * t597 * t5273 * t562;
    let t17067 = 4.0 / 45.0 * t639 * t642 * t643 * t16978;
    let t17068 = t1627 * t5464;
    let t17069 = 32.0 / 135.0 * t17068;
    let t17070 = t1627 * t4998;
    let t17071 = 64.0 / 45.0 * t17070;
    let t17072 = t5467 * t1631;
    (t17058, t17063, t17067, t17069, t17071, t17072)
}
