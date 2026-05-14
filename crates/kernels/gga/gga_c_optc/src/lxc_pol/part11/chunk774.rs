//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 774/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk774<F: Float>(t4333: F, t4363: F, t1128: F, t5313: F, t1121: F, t2586: F, t5297: F, t1133: F, t140: F, t5255: F, t871: F, t464: F, t3061: F, t5218: F, t5202: F, t8700: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15330 = t4363 * t4333;
    let t15332 = t1128 * t5313;
    let t15333 = t1121 * t15332;
    let t15335 = t2586 * t5297;
    let t15336 = t1133 * t15335;
    let t15354 = t5255 * t871 * t140;
    let t15355 = t464 * t15354;
    let t15374 = t5218 * t3061;
    let t15381 = t5202 * t8700;
    (t15330, t15332, t15333, t15335, t15336, t15354, t15355, t15374, t15381)
}
