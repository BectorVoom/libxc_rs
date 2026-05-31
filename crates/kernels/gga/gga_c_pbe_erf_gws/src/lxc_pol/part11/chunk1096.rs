//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1096/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1096<F: Float>(t40324: F, t40327: F, t40358: F, t40361: F, t1037: F, t41638: F, t10908: F, t1820: F, t1885: F, t3345: F, t1010: F, t40329: F) -> (F, F, F, F, F, F, F) {
    let t47565 = F::cast_from(256.0_f64) / F::cast_from(243.0_f64) * t40324;
    let t47566 = F::cast_from(64.0_f64) / F::cast_from(15.0_f64) * t40327;
    let t47567 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t40358;
    let t47568 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40361;
    let t47570 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t41638 * t1037;
    let t47574 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t1820 * t1885 * t10908 * t3345;
    let t47576 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40329 * t1010;
    (t47565, t47566, t47567, t47568, t47570, t47574, t47576)
}
