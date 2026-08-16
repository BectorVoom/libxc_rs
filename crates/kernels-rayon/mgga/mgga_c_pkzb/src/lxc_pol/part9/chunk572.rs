//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 572/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk572(t2369: f64, t2371: f64, t758: f64, t2099: f64, t922: f64, t918: f64, t178: f64, t916: f64, t915: f64) -> (f64, f64, f64, f64, f64) {
    let t2372 = t2369 * t2371;
    let t2373 = t758 * t2372;
    let t2376 = t2099 * t922;
    let t2377 = t918 * t2376;
    let t2379 = t916 * t178;
    let t2380 = t915 * t2379;
    (t2372, t2373, t2376, t2377, t2380)
}
