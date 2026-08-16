//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 455/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk455(t2520: f64, t612: f64, t1936: f64, t889: f64, t6: f64, t891: f64, t2503: f64, t1944: f64, t320: f64, t1: f64, t314: f64) -> (f64, f64, f64, f64, f64) {
    let t2521 = t2520 * t612;
    let t2524 = t889 * t1936;
    let t2525 = t891 * t6;
    let t2526 = t2503 * t2525;
    let t2529 = t320 * t1944;
    let t2530 = t314 * t1;
    (t2521, t2524, t2526, t2529, t2530)
}
