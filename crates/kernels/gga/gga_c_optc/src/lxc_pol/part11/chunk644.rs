//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 644/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk644<F: Float>(t106: F, t1147: F, t1550: F, t3170: F, t4403: F, t470: F, t5344: F, t5351: F, t5430: F, t115: F, t5274: F, t5: F, t497: F, t2844: F, t2866: F, t4068: F, t4117: F, t5108: F, t5112: F, t5115: F, t5146: F, t5149: F, t5152: F) -> (F, F, F, F) {
    let t5434 = 0.27818116767324025134e1 * t106 * t5344 * t470 - 0.55636233534648050268e1 * t106 * t4403 * t1550 + 0.55636233534648050268e1 * t106 * t3170 * t5351 - 0.27818116767324025134e1 * t106 * t1147 * t5430;
    let t5439 = t5274 * t115;
    let t5440 = t5439 * t5;
    let t5441 = t5440 * t497;
    let t5454 = t2844 + 0.12925555555555555555e1 * t4068 - 0.12925555555555555555e1 * t5108 + 0.38776666666666666666e1 * t5112 - 0.19388333333333333333e1 * t5115 + t2866 + 0.1642e-2 * t4117 - 0.4105e-3 * t5146 + 0.2463e-2 * t5149 - 0.12315e-2 * t5152;
    (t5434, t5440, t5441, t5454)
}
