//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1407/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1407<F: Float>(t28030: F, t1220: F, t176: F, t26175: F, t26179: F, t26282: F, t26337: F, t275: F, t277: F, t28002: F, t28010: F, t28017: F, t28020: F, t28023: F, t28026: F, t28028: F, t3274: F, t3284: F, t498: F, t8431: F, t8436: F, t914: F, t95: F, sigma2: F) -> F {
    let t28031 = F::new(1.0) / t28030;
    let t28039 = t176 * t28002 * t275 * sigma2 * t498 / F::new(2.0) + F::new(56.0) / F::new(27.0) * t3274 * t8431 + F::new(140.0) / F::new(81.0) * t1220 * t914 * t28010 * t26337 + F::new(2.0) / F::new(3.0) * t3274 * t8436 - F::new(4.0) / F::new(9.0) * t28017 + F::new(20.0) / F::new(81.0) * t28020 - F::new(2.0) / F::new(9.0) * t28023 - F::new(8.0) / F::new(27.0) * t28026 - t26175 - t26179 - F::cast_from(0.15506928860942058298e-1_f64) * t95 * t277 * t28028 * t28031 + F::new(8.0) * t1220 * t914 * t3284 * t26282;
    t28039
}
