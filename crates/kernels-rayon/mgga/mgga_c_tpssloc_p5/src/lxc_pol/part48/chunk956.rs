//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 956/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk956(t113023: f64, t113032: f64, t114699: f64, t114704: f64, t114708: f64, t114741: f64, t114746: f64, t114750: f64, t114752: f64, t226: f64, t235: f64, t2613: f64, t31397: f64, t808: f64, t8560: f64) -> f64 {
    let t114754 = -0.3289868133696452873e-1_f64 * t114699 - t113023 + 0.3289868133696452873e-1_f64 * t114704 + 0.82246703342411321825e-2_f64 * t114708 + t2613 * t8560 + 2.0_f64 * t808 * t31397 + t226 * t235 * t114741 + 0.49348022005446793095e-1_f64 * t114746 - 0.82246703342411321825e-2_f64 * t114750 + 0.38381794893125283518e-1_f64 * t114752 - t113032;
    t114754
}
