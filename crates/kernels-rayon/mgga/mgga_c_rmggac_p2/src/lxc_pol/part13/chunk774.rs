//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 774/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk774(t22: f64, t4616: f64, t326: f64, t262: f64, t265: f64, t7835: f64, t876: f64, t2078: f64, t26: f64, t3814: f64, t36: f64, t2064: f64, t839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35928 = t4616 * t22;
    let t35929 = t326 * t35928;
    let t35937 = t7835 * t262 * t265 * t876;
    let t35959 = t2078 * t26;
    let t35960 = t3814 * t35959;
    let t35972 = t4616 * t36;
    let t35979 = t2064 * t839;
    (t35929, t35937, t35959, t35960, t35972, t35979)
}
