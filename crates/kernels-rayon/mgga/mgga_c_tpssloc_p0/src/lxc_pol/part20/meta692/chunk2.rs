//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2637/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2637(t15816: f64, t225: f64, t11608: f64, t11613: f64, t11925: f64, t11928: f64, t1235: f64, t1252: f64, t14980: f64, t15425: f64, t15787: f64, t15797: f64, t15803: f64, t3481: f64, t3487: f64, t3593: f64, t3600: f64, t3631: f64, t466: f64, t4945: f64, t498: f64, t5052: f64, t5055: f64, t5060: f64, t5089: f64, t53529: f64) -> f64 {
    let t53703 = t15816 * t225;
    let t53729 = 3.0_f64 * t1235 * t15425 * t498 + 3.0_f64 * t3481 * t498 * t5052 + t466 * t498 * t53529 - 6.0_f64 * t11608 * t4945 - 6.0_f64 * t11608 * t5055 + 12.0_f64 * t11613 * t5060 - 3.0_f64 * t11925 * t5089 + 6.0_f64 * t11928 * t5060 - 6.0_f64 * t1252 * t53703 + 6.0_f64 * t14980 * t3600 - 3.0_f64 * t14980 * t3631 - 3.0_f64 * t15787 * t3487 + 6.0_f64 * t15797 * t3600 + 6.0_f64 * t15803 * t3593;
    t53729
}
