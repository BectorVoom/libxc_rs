//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 795/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk795(t4963: f64, t888: f64, t874: f64, t1: f64, t10975: f64, t2269: f64, t297: f64, t1325: f64, t3813: f64) -> (f64, f64, f64, f64) {
    let t14326 = t888 * t4963;
    let t14327 = t874 * t14326;
    let t14329 = t10975 * t1;
    let t14330 = t297 * t2269;
    let t14339 = t3813 * t1325;
    (t14327, t14329, t14330, t14339)
}
