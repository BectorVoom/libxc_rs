//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 662/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk662<F: Float>(t106: F, t1147: F, t1550: F, t3170: F, t4403: F, t470: F, t5344: F, t5351: F, t5430: F, t115: F, t5274: F, t5: F) -> (F, F) {
    let t5434 = F::cast_from(0.27818116767324025134e1_f64) * t106 * t5344 * t470 - F::cast_from(0.55636233534648050268e1_f64) * t106 * t4403 * t1550 + F::cast_from(0.55636233534648050268e1_f64) * t106 * t3170 * t5351 - F::cast_from(0.27818116767324025134e1_f64) * t106 * t1147 * t5430;
    let t5439 = t5274 * t115;
    let t5440 = t5439 * t5;
    (t5434, t5440)
}
