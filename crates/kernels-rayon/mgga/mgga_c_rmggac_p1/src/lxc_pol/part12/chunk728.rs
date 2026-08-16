//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 728/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk728(t7203: f64, t899: f64, t20: f64, t4764: f64, t132: f64, t1327: f64, t20925: f64, t253: f64, t7321: f64, t4765: f64, t49: f64, t7322: f64) -> (f64, f64, f64, f64) {
    let t34738 = t899 * t7203;
    let t34747 = t20 * t4764;
    let t34750 = t132 * t1327;
    let t34752 = t253 * t34747 * t7321 * t20925 * t34750;
    let t34753 = 0.10260057759007034251e-5_f64 * t34752;
    let t34755 = t4765 * t7322 * t49;
    (t34738, t34750, t34753, t34755)
}
