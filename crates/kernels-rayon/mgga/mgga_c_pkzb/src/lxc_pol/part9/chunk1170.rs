//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1170/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1170(t16940: f64, t1542: f64, t2607: f64, t16810: f64, t16813: f64, t16822: f64, t16825: f64, t16938: f64, t16946: f64, t16950: f64, t20363: f64, t20365: f64, t20367: f64, t20369: f64, t20371: f64, t20373: f64, t20375: f64, t20376: f64) -> (f64, f64, f64) {
    let t20377 = 192.0_f64 * t16940;
    let t20378 = t1542 * t2607;
    let t20379 = 60.0_f64 * t20378;
    let t20380 = -t20363 + t16810 - t16813 - t16822 + t20365 - t20367 - t20369 + t20371 + t20373 - t20375 + t16825 - t20376 + t16938 + t20377 + t16946 + t16950 + t20379;
    (t20377, t20379, t20380)
}
