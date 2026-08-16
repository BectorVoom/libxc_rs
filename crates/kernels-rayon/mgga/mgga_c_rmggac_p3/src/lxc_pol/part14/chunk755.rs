//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 755/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk755(t35496: f64, t1223: f64, t211: f64, t1965: f64, t1977: f64, t1982: f64, t2004: f64, t7939: f64, t118: f64, t1986: f64, t338: f64, t352: f64, t495: f64) -> (f64, f64, f64, f64, f64) {
    let t35497 = 0.63245127235888530833e-7_f64 * t35496;
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35516 = t7939 * t2004;
    let t35523 = t1986 * t118 * t338 * t495 * t352;
    (t35497, t35512, t35514, t35516, t35523)
}
