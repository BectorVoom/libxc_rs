//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 397/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk397<F: Float>(t1966: F, t1979: F, t1987: F, t2370: F, t2384: F, t2387: F, t2396: F, t240: F, t2597: F, t2605: F, t2609: F, t764: F) -> F {
    let t2618 = -t2370 + t2384 + t240 * (-F::cast_from(0.3109e-1_f64) * t2597 * t764 + F::cast_from(1.0_f64) * t1966 * t2605 + t2370 - t2384 - F::cast_from(0.19751789702565206229e-1_f64) * t2387 + F::cast_from(0.58482233974552040708e0_f64) * t1979 * t2609) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t2387 - F::cast_from(0.58482233974552040708e0_f64) * t1987 * t2396;
    t2618
}
