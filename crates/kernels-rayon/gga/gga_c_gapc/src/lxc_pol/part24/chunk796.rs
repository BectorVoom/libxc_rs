//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 796/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk796(t9507: f64, t9508: f64, t8700: f64, t889: f64, t3397: f64, t1068: f64, t2387: f64, t322: f64, t3307: f64, t913: f64, t3288: f64, t7577: f64) -> (f64, f64, f64, f64, f64) {
    let t9509 = t9507 * t9508;
    let t9512 = t889 * t8700;
    let t9513 = t9512 * t3397;
    let t9515 = t2387 * t1068;
    let t9516 = t9515 * t322;
    let t9518 = t3307 * t913;
    let t9520 = t3288 * t7577;
    (t9509, t9513, t9516, t9518, t9520)
}
