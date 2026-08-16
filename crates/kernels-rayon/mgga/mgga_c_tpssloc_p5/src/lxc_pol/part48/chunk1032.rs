//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1032/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1032(t112: f64, t32392: f64, t111: f64, t8843: f64, t114483: f64, t114489: f64, t114494: f64, t114500: f64, t115990: f64, t115995: f64, t116000: f64, t116004: f64, t116006: f64, t116008: f64, t117662: f64, t2319: f64, t23917: f64, t24478: f64, t24481: f64, t24969: f64, t24972: f64, t577: f64, t671: f64, t7056: f64, t7235: f64, t7423: f64, t85416: f64) -> f64 {
    let t117672 = t32392 * t112;
    let t117687 = t8843 * t111;
    let t117690 = 27.0_f64 * t117672 * t671 + 0.135e2_f64 * t7423 * t23917 + t115990 + t114483 + t114489 + t115995 + t114494 + 54.0_f64 * t85416 * t7235 + t116000 + 27.0_f64 * t24969 * t7056 + t114500 + 0.45e1_f64 * t117662 * t577 + 54.0_f64 * t24972 * t24478 + 27.0_f64 * t24972 * t24481 + 27.0_f64 * t117687 * t2319 + t116004 + t116006 + t116008;
    t117690
}
