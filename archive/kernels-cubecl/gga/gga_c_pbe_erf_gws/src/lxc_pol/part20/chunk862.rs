//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 862/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk862<F: Float>(t3205: F, t329: F, t838: F, t3209: F, t3214: F, t4414: F, t1164: F, t2242: F, t3123: F, t6184: F, t3133: F, t6183: F) -> (F, F, F, F, F, F) {
    let t8801 = t329 * t838 * t3205;
    let t8803 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t8801 * t3209;
    let t8810 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t4414 * t3214;
    let t8818 = t2242 * t1164;
    let t8823 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3123 * t6184;
    let t8824 = t6183 * t3133;
    (t8801, t8803, t8810, t8818, t8823, t8824)
}
