//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 990/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk990<F: Float>(t41218: F, t10848: F, t3504: F, t41223: F, t12660: F, t7130: F, t32202: F, t47874: F, t47878: F, t47882: F, t47886: F, t47888: F, t47890: F, t47892: F, t32215: F, t3479: F, t3555: F) -> (F, F, F, F, F, F, F, F) {
    let t47893 = 64.0 / 15.0 * t41218;
    let t47895 = 16.0 / 15.0 * t10848 * t3504;
    let t47896 = 64.0 / 45.0 * t41223;
    let t47898 = 16.0 / 5.0 * t7130 * t12660;
    let t47899 = 16.0 / 81.0 * t32202;
    let t47900 = -t47874 + t47878 + t47882 + t47886 - t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899;
    let t47902 = 16.0 / 45.0 * t32215;
    let t47904 = 4.0 / 5.0 * t3479 * t3555;
    (t47893, t47895, t47896, t47898, t47899, t47900, t47902, t47904)
}
