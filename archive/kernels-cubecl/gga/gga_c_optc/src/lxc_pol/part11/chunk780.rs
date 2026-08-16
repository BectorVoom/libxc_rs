//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 780/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk780<F: Float>(t2182: F, t4661: F, t2144: F, t4703: F, t4706: F, t7018: F, t2120: F, t4690: F, t4694: F, t4681: F, t7037: F, t2164: F, t4712: F) -> (F, F, F, F, F, F, F) {
    let t13366 = t2182 * t4661;
    let t13368 = t2144 * t4703;
    let t13373 = t7018 * t4706;
    let t13376 = t2120 * t4690;
    let t13378 = t2120 * t4694;
    let t13380 = t7037 * t4681;
    let t13390 = t2164 * t4712;
    (t13366, t13368, t13373, t13376, t13378, t13380, t13390)
}
