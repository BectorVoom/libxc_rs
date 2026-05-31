//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1123/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1123<F: Float>(t12564: F, t2612: F, t11032: F, t3500: F, t41218: F, t10848: F, t3504: F, t41223: F, t12660: F, t7130: F, t32202: F, t47874: F, t47878: F, t47882: F, t47886: F, t47888: F) -> (F, F, F, F, F, F, F, F) {
    let t47890 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t2612 * t12564;
    let t47892 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t11032 * t3500;
    let t47893 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t41218;
    let t47895 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10848 * t3504;
    let t47896 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t41223;
    let t47898 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t7130 * t12660;
    let t47899 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t32202;
    let t47900 = -t47874 + t47878 + t47882 + t47886 - t47888 - t47890 - t47892 - t47893 - t47895 + t47896 - t47898 - t47899;
    (t47890, t47892, t47893, t47895, t47896, t47898, t47899, t47900)
}
