//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 453/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk453(t1975: f64, t961: f64, t181: f64, t2190: f64, t329: f64, t818: f64, t314: f64) -> (f64, f64, f64, f64) {
    let t2486 = t1975 * t961;
    let t2489 = t181 * t2190;
    let t2492 = t818 * t329;
    let t2493 = t2492 * t314;
    (t2486, t2489, t2492, t2493)
}
