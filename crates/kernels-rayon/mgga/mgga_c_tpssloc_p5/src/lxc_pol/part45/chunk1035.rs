//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1035/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1035(t114335: f64, t22574: f64, t24432: f64, t112547: f64, t115721: f64, t115725: f64, t115727: f64, t115728: f64, t115732: f64, t115738: f64, t115743: f64, t115748: f64, t115750: f64, t115752: f64, t115754: f64, t1266: f64, t1393: f64, t2096: f64, t23958: f64, t24028: f64, t31246: f64, t31700: f64, t31722: f64, t7218: f64, t8450: f64) -> f64 {
    let t115757 = 6.0_f64 * t22574 * t24432 * t114335;
    let t115758 = t112547 * t2096 - 2.0_f64 * t1266 * t31700 + 2.0_f64 * t1393 * t31722 + 6.0_f64 * t23958 * t8450 - 2.0_f64 * t24028 * t8450 + 2.0_f64 * t31246 * t7218 + t115721 - t115725 - t115727 - t115728 - t115732 - t115738 - t115743 - t115748 + t115750 - t115752 - t115754 - t115757;
    t115758
}
