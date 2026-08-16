//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 759/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk759(t1341: f64, t357: f64, t638: f64, t7310: f64, t7254: f64, t7364: f64, t7243: f64, t1326: f64, t2016: f64, t7551: f64, t2049: f64, t35253: f64, t7760: f64) -> (f64, f64, f64, f64, f64) {
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    let t35654 = t7254 * t7243;
    let t35688 = t2016 * t7551 * t1326;
    let t35691 = t35688 * t2049 * t35253 * t7760;
    (t35633, t35637, t35654, t35688, t35691)
}
