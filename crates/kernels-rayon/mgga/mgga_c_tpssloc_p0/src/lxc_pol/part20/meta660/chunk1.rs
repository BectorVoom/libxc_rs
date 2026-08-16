//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2465/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2465(t1022: f64, t1058: f64, t1060: f64, t1061: f64, t11037: f64, t11046: f64, t11051: f64, t11078: f64, t14526: f64, t14595: f64, t14627: f64, t14630: f64, t14645: f64, t14651: f64, t3180: f64, t3186: f64, t3188: f64, t3197: f64, t4669: f64, t4673: f64, t4677: f64, t4678: f64, t4680: f64, t50535: f64, t50540: f64) -> f64 {
    let t50560 = 3.0_f64 * t1022 * t1058 * t1060 * t14526 + 3.0_f64 * t11046 * t14630 * t4677 + 3.0_f64 * t11046 * t14630 * t4680 + 6.0_f64 * t14595 * t3186 * t4673 + 6.0_f64 * t3186 * t3188 * t50540 + 3.0_f64 * t1061 * t50535 - 3.0_f64 * t11037 * t14627 + 3.0_f64 * t11051 * t4678 + 3.0_f64 * t11078 * t4669 + 6.0_f64 * t14645 * t3180 + 3.0_f64 * t14651 * t3197;
    t50560
}
