//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 925/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk925(t43: f64, t16900: f64, t16925: f64, t16958: f64, t17320: f64, t16231: f64, t16236: f64, t8428: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t17322 = t16900 + t16925 + t16958 + t17320;
    let t17329 = piecewise3(t44, 0.0_f64, t16231);
    let t17336 = t8428 * t16236;
    (t17322, t17329, t17336)
}
