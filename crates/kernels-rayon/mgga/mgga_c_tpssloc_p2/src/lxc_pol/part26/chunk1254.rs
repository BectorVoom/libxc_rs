//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1254/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1254(t22674: f64, t22934: f64, t6897: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t3911: f64, t22935: f64, t6883: f64, t22667: f64, t1987: f64, t81144: f64, t9537: f64) -> (f64, f64, f64, f64, f64) {
    let t81379 = t6897 * t22674 * t22934;
    let t81386 = t22633 * t22635 * t1377 * t3911 * t1307;
    let t81393 = t6883 * t22935;
    let t81395 = t6883 * t22667;
    let t81398 = t81144 * t9537 * t1987;
    (t81379, t81386, t81393, t81395, t81398)
}
