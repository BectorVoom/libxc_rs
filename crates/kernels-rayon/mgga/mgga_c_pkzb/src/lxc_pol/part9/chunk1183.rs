//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1183/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1183(t1676: f64, t7177: f64, t16825: f64, t16938: f64, t16946: f64, t16950: f64, t20371: f64, t20373: f64, t20375: f64, t20376: f64, t20377: f64, t20379: f64, t2718: f64, t5191: f64, t6758: f64) -> (f64, f64) {
    let t20615 = t7177 * t1676;
    let t20623 = 18.0_f64 * t2718 * t5191 * t6758 + t16825 + t16938 + t16946 + t16950 + t20371 + t20373 - t20375 - t20376 + t20377 + t20379;
    (t20615, t20623)
}
