//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 766/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk766(t326: f64, t35928: f64, t2078: f64, t26: f64, t3814: f64, t36: f64, t4616: f64, t34805: f64, t648: f64, t305: f64, t35590: f64, t2115: f64, t35876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35929 = t326 * t35928;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t35972 = t4616 * t36;
    let t36034 = t648 * t34805;
    let t36035 = 0.15556658869458454171e0_f64 * t36034;
    let t36058 = t305 * t35590;
    let t36088 = t2115 * t35876;
    (t35929, t35959, t35960, t35972, t36035, t36058, t36088)
}
