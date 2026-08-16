//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 759/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk759(t1040: f64, t8888: f64, t1022: f64, t1980: f64, t5462: f64, t641: f64, t1: f64, t1736: f64, t102: f64, t1648: f64, t1894: f64, t1026: f64, t1846: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8889 = t8888 * t1040;
    let t8891 = t1022 * t1980;
    let t8893 = t5462 * t641;
    let t8894 = t1736 * t1;
    let t8895 = t8894 * t102;
    let t8897 = t8895 * t1648 * t1894;
    let t8898 = t8893 * t8897;
    let t8900 = t1846 * t1026;
    (t8889, t8891, t8893, t8895, t8898, t8900)
}
