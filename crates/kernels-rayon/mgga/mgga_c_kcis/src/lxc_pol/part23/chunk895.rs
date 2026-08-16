//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 895/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk895(t12274: f64, t2013: f64, t3738: f64, t5757: f64, t1464: f64, t3722: f64, t5756: f64, t1395: f64, t11776: f64, t2012: f64, t3728: f64, t5761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16756 = t12274 * t2013;
    let t16758 = t3738 * t5757;
    let t16759 = t1464 * t16758;
    let t16761 = t5756 * t3722;
    let t16762 = t1395 * t16761;
    let t16763 = t1464 * t16762;
    let t16765 = t11776 * t2012;
    let t16766 = t1464 * t16765;
    let t16768 = t3728 * t5761;
    (t16756, t16759, t16761, t16763, t16766, t16768)
}
