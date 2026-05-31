//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1291/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1291<F: Float>(t1289: F, t3138: F, t3142: F, t8497: F, t8512: F, t8518: F, t10200: F, t8498: F, t10195: F, t10199: F, t10205: F, t10212: F, t1240: F, t1318: F, t2002: F, t20560: F, t20563: F, t20574: F, t20578: F, t20579: F, t23909: F, t27852: F, t27871: F, t3: F, t3139: F, t3140: F, t3271: F, t675: F, t8441: F, t8502: F, t8506: F, t8511: F, t8513: F, t8514: F, t8519: F, t8521: F, t8526: F) -> F {
    let t27996 = t3138 * t8497 * t1289 * t3142;
    let t28005 = t8512 * t1289;
    let t28009 = t8518 * t1289;
    let t28014 = t3138 * t8498 * t10200;
    let t28043 = t3138 * t23909 * t10212 / F::cast_from(6.0_f64) - t27996 / F::cast_from(36.0_f64) - t3138 * t3139 * t3271 * t3142 / F::cast_from(12.0_f64) - t3138 * t10195 * t8506 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t8511 * t28005 * t8514 + t3138 * t28009 * t8521 / F::cast_from(6.0_f64) - t28014 / F::cast_from(36.0_f64) + t8526 * t3140 * t27871 / F::cast_from(16.0_f64) - t3138 * t8502 * t10200 / F::cast_from(12.0_f64) - t3138 * t3140 * t10199 * t2002 / F::cast_from(24.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t8511 * t8513 * t27852 + t3138 * t8519 * t1318 * t3 * t675 / F::cast_from(6.0_f64) + t8526 * t8502 * t10205 / F::cast_from(8.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t1240 * t8441 + t20560 / F::cast_from(48.0_f64) + t20563 / F::cast_from(96.0_f64) - F::cast_from(5.0_f64) / F::cast_from(144.0_f64) * t20574 + t20578 + t20579 / F::cast_from(96.0_f64);
    t28043
}
