//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 775/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk775(t1734: f64, t9066: f64, t1743: f64, t5218: f64, t122: f64, t1845: f64, t2995: f64, t3001: f64, t3060: f64, t3008: f64, t102: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9067 = t1734 * t9066;
    let t9068 = t1743 * t9067;
    let t9069 = t9068 * t5218;
    let t9071 = t1845 * t122;
    let t9072 = t9071 * t2995;
    let t9073 = t9072 * t3001;
    let t9075 = t3060 * t2995;
    let t9076 = t9075 * t3008;
    let t9078 = t102 * t505;
    (t9067, t9068, t9069, t9071, t9073, t9076, t9078)
}
