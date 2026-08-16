//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 755/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk755(t35496: f64, t1223: f64, t211: f64, t1965: f64, t1977: f64, t1982: f64, t265: f64, t4789: f64, t638: f64, t71: f64, t7311: f64, t321: f64, t7817: f64) -> (f64, f64, f64, f64, f64) {
    let t35497 = 0.63245127235888530833e-7_f64 * t35496;
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    let t35566 = 0.24390119833260022651e-2_f64 * t35565;
    let t35583 = t7817 * t321;
    (t35497, t35512, t35514, t35566, t35583)
}
