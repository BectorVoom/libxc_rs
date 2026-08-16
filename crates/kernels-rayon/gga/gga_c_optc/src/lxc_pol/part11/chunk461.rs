//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 461/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk461(t222: f64, t2278: f64, t381: f64, t1113: f64, t136: f64) -> (f64, f64, f64) {
    let t2865 = t222 * t2278 * t381;
    let t2866 = 0.20525e-2_f64 * t2865;
    let t2869 = t136 * t1113;
    (t2865, t2866, t2869)
}
