//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2215/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2215(t81066: f64, t1307: f64, t1352: f64, t1834: f64, t22633: f64, t6976: f64, t16037: f64, t1992: f64, t22897: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64) -> (f64, f64, f64, f64, f64) {
    let t90903 = 0.16449340668482264365e-1_f64 * t81066;
    let t90907 = t22633 * t6976 * t1834 * t1307 * t1352;
    let t90910 = t1992 * t22897 * t16037;
    let t90912 = t81159 * t26423;
    let t90913 = 0.76763589786250567036e-1_f64 * t90912;
    let t90914 = t22839 * t215;
    (t90903, t90907, t90910, t90913, t90914)
}
