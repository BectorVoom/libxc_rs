//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 881/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk881(t8460: f64, t8461: f64, t3104: f64, t3126: f64, t4357: f64, t1122: f64) -> (f64, f64, f64, f64) {
    let t8462 = t8460 * t8461;
    let t8465 = t3104 * t3126;
    let t8466 = t8465 * t4357;
    let t8469 = t3104 * t1122;
    (t8462, t8465, t8466, t8469)
}
