//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1229/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1229(t1433: f64, t31: f64, t607: f64, t113875: f64, t12571: f64, t31009: f64, t33106: f64, t645: f64, t8513: f64, t6504: f64, t641: f64, t7431: f64) -> (f64, f64, f64, f64, f64) {
    let t119901 = t1433 * t31 * t607;
    let t119902 = t113875 * t119901;
    let t119905 = t12571 * t31009;
    let t119909 = t8513 * t33106 * t645;
    let t119913 = t8513 * t33106 * t6504;
    let t119917 = t8513 * t7431 * t641;
    (t119902, t119905, t119909, t119913, t119917)
}
