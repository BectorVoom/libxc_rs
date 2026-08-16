//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2964/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2964(t1412: f64, t808: f64, t13927: f64, t48862: f64, t1389: f64, t14224: f64, t46835: f64, t13769: f64, t2453: f64, t547: f64, t9794: f64, t14230: f64, t2735: f64, t46801: f64) -> (f64, f64, f64, f64, f64) {
    let t48863 = t808 * t1412;
    let t48865 = t48862 * t48863 * t13927;
    let t48868 = t46835 * t1389 * t14224;
    let t48872 = t2453 * t547 * t9794 * t13769;
    let t48876 = t2735 * t46801 * t1389 * t14230;
    (t48863, t48865, t48868, t48872, t48876)
}
