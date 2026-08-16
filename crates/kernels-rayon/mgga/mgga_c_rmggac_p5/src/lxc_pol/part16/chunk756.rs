//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 756/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk756(t1223: f64, t211: f64, t1965: f64, t1977: f64, t1982: f64, t265: f64, t4789: f64, t638: f64, t71: f64, t7311: f64, t321: f64, t7817: f64) -> (f64, f64, f64, f64) {
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    let t35583 = t7817 * t321;
    (t35512, t35514, t35565, t35583)
}
