//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1082/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1082(t42296: f64, t42297: f64, t42298: f64, t42299: f64, t42300: f64, t42301: f64, t7886: f64, t8197: f64, t9501: f64, t9600: f64, t9601: f64, t9035: f64) -> (f64, f64) {
    let t42302 = -t8197 + t9501 + t42296 + t7886 - t42297 + t42298 + t9600 + t9601 + t42299 + t42300 - t42301;
    let t42306 = 0.11974241701863808564e0_f64 * t9035;
    (t42302, t42306)
}
