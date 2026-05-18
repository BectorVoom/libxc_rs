//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 648/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk648<F: Float>(t106: F, t1147: F, t1182: F, t3160: F, t3164: F, t3170: F, t3171: F, t3264: F, t470: F, t1207: F, t176: F, t1219: F) -> (F, F, F) {
    let t3268 = F::new(0.27818116767324025134e1) * t106 * t3160 * t470 - F::new(0.55636233534648050268e1) * t106 * t3164 * t1182 + F::new(0.55636233534648050268e1) * t106 * t3170 * t3171 - F::new(0.27818116767324025134e1) * t106 * t1147 * t3264;
    let t3273 = t176 * t1207;
    let t3274 = t3273 * t1219;
    (t3268, t3273, t3274)
}
