//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 909/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk909<F: Float>(t3023: F, t35: F, t572: F, t6007: F, t6010: F, t6013: F, t6015: F, t6017: F, t6019: F, t7933: F, t7936: F, t7938: F, t7943: F, t7948: F, t7953: F, t7958: F, t7962: F, t7966: F, t7971: F, t7975: F, t7979: F) -> F {
    let t7983 = -t6010 - F::cast_from(4.0_f64) / F::cast_from(243.0_f64) * t6013 + t6015 / F::cast_from(243.0_f64) - t6017 / F::cast_from(81.0_f64) + t6019 / F::cast_from(162.0_f64) - F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t7933 + t7936 - t7938 + F::cast_from(11.0_f64) / F::cast_from(81.0_f64) * t7943 - F::cast_from(5.0_f64) / F::cast_from(243.0_f64) * t572 * t7948 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t572 * t7953 - F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t3023 * t7958 - t572 * t7962 / F::cast_from(81.0_f64) - t572 * t7966 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t3023 * t7971 + t572 * t7975 / F::cast_from(27.0_f64) - t35 * t6007 * t7979 / F::cast_from(27.0_f64);
    t7983
}
