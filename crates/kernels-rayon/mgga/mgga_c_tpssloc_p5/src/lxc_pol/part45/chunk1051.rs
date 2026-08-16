//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1051/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1051(t114456: f64, t114513: f64, t114515: f64, t114517: f64, t114520: f64, t114525: f64, t114527: f64, t114529: f64, t114531: f64, t115972: f64, t115978: f64, t115980: f64, t2363: f64, t23880: f64, t23917: f64, t24478: f64, t31795: f64, t577: f64, t7010: f64, t7235: f64, t83980: f64, t8508: f64) -> f64 {
    let t115981 = 0.135e2_f64 * t7010 * t23917 + t114513 + t114515 + t114517 + t114520 + t114456 + 54.0_f64 * t23880 * t24478 + t8508 + t114525 + t114527 + t114529 + t114531 + 0.135e2_f64 * t31795 * t2363 + 0.45e1_f64 * t115972 * t577 + 54.0_f64 * t83980 * t7235 + t115978 + t115980;
    t115981
}
