//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 968/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk968(t1459: f64, t8286: f64, t475: f64, t4855: f64, t2953: f64, t3652: f64, t1603: f64, t3639: f64, t1006: f64, t1005: f64, t3946: f64, t1577: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11248 = t8286 * t1459;
    let t11249 = t475 * t4855;
    let t11250 = t11248 * t11249;
    let t11252 = t2953 * t3652;
    let t11254 = t3639 * t1603;
    let t11255 = t1006 * t11254;
    let t11257 = t1005 * t3946;
    let t11258 = t3639 * t1577;
    (t11248, t11249, t11250, t11252, t11254, t11255, t11257, t11258)
}
