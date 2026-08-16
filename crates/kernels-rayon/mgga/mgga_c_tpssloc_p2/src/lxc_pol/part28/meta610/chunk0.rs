//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1921/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1921(t22724: f64, t26436: f64, t1307: f64, t1352: f64, t1834: f64, t22633: f64, t6976: f64, t16037: f64, t1992: f64, t22897: f64, t26423: f64, t81159: f64) -> (f64, f64, f64, f64) {
    let t90900 = t22724 * t26436;
    let t90907 = t22633 * t6976 * t1834 * t1307 * t1352;
    let t90910 = t1992 * t22897 * t16037;
    let t90912 = t81159 * t26423;
    (t90900, t90907, t90910, t90912)
}
