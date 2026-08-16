//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 397/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk397(t1966: f64, t1979: f64, t1987: f64, t2370: f64, t2384: f64, t2387: f64, t2396: f64, t240: f64, t2597: f64, t2605: f64, t2609: f64, t764: f64) -> f64 {
    let t2618 = -t2370 + t2384 + t240 * (-0.3109e-1_f64 * t2597 * t764 + 1.0_f64 * t1966 * t2605 + t2370 - t2384 - 0.19751789702565206229e-1_f64 * t2387 + 0.58482233974552040708e0_f64 * t1979 * t2609) + 0.19751789702565206229e-1_f64 * t240 * t2387 - 0.58482233974552040708e0_f64 * t1987 * t2396;
    t2618
}
