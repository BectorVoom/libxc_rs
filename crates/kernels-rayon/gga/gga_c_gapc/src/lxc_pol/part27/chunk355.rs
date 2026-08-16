//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 355/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk355(t1599: f64, t514: f64, t19: f64, t203: f64, t147: f64, t567: f64) -> (f64, f64, f64, f64) {
    let t1600 = t514 * t1599;
    let t1601 = t203 * t19;
    let t1602 = t147 * t567;
    let t1603 = t1601 * t1602;
    (t1600, t1601, t1602, t1603)
}
