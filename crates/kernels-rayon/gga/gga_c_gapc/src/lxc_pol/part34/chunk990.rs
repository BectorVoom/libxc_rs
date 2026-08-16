//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 990/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk990(t11451: f64, t5126: f64, t11450: f64, t1936: f64, t5462: f64, t144: f64, t1453: f64, t5526: f64, t674: f64, t5708: f64, t612: f64, t5713: f64, t9066: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11452 = t11451 * t5126;
    let t11453 = t11450 * t11452;
    let t11455 = t5462 * t1936;
    let t11456 = t1453 * t144;
    let t11458 = t11456 * t674 * t5526;
    let t11459 = t11455 * t11458;
    let t11463 = t5708 * t612;
    let t11465 = t9066 * t144 * t5713;
    (t11452, t11453, t11455, t11458, t11459, t11463, t11465)
}
