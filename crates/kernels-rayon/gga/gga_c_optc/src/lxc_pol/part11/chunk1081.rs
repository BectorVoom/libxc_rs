//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1081/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1081(t13061: f64, t1990: f64, t13110: f64, t544: f64, t1797: f64, t4: f64, t4579: f64, t4665: f64, t7061: f64, t4715: f64, t7022: f64, t4712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38346 = t13061 * t1990;
    let t38368 = t544 * t13110;
    let t38375 = t4579 * t4 * t1797;
    let t38433 = t7061 * t4665;
    let t38444 = t7022 * t4715;
    let t38446 = t7022 * t4712;
    (t38346, t38368, t38375, t38433, t38444, t38446)
}
