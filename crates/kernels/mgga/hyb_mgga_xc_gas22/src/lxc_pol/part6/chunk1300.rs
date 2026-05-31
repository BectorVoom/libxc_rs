//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1300/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1300<F: Float>(t24143: F, t3: F, t4002: F, t6012: F, t10288: F, t10293: F, t2002: F, t2028: F, t20292: F, t24140: F, t24142: F, t24149: F, t24154: F, t24158: F, t24161: F, t24163: F, t24186: F, t24205: F, t27275: F, t3171: F, t3925: F, t572: F, t8296: F) -> F {
    let t28258 = t24143 * t3;
    let t28268 = t6012 * t4002;
    let t28274 = -F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t24140 - F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t572 * t8296 * t10288 * t2002 - F::cast_from(40.0_f64) / F::cast_from(729.0_f64) * t572 * t24205 * t20292 * t3925 * t2028 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t572 * t3171 * t10293 * t2002 + F::cast_from(40.0_f64) / F::cast_from(243.0_f64) * t27275 * t24154 * t28258 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t27275 * t24149 * t28258 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t27275 * t24142 * t28258 - F::cast_from(4.0_f64) / F::cast_from(729.0_f64) * t28268 - F::cast_from(8.0_f64) / F::cast_from(243.0_f64) * t24158 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t24161 + F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t24163 + F::cast_from(28.0_f64) / F::cast_from(729.0_f64) * t24186;
    t28274
}
