//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1124/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1124(t16152: f64, t33673: f64, t33884: f64, t7204: f64, t11522: f64, t18866: f64, t9396: f64, t11941: f64, t9652: f64, t3375: f64, t33757: f64, t33582: f64, t3789: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33930 = t33673 * t16152;
    let t33932 = t7204 * t33884;
    let t33935 = t18866 * t11522 * t9396;
    let t33937 = t9652 * t11941;
    let t33939 = t33757 * t3375;
    let t33941 = t33582 * t3789;
    (t33930, t33932, t33935, t33937, t33939, t33941)
}
