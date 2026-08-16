//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 562/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk562(t14319: f64, t14324: f64, t14472: f64, t14473: f64, t14496: f64, t14500: f64, t14501: f64, t14505: f64, t14508: f64, t14511: f64, t14514: f64, t14528: f64, t305: f64, t326: f64) -> f64 {
    let t14530 = t14472 - 0.59871208509319042821e-1_f64 * t326 * t14473 + 0.19957069503106347607e-1_f64 * t14496 - t14500 + 0.59871208509319042821e-1_f64 * t305 * t14501 - t14505 + t14508 - t14319 + t14324 - t14511 - t14514 + t14528;
    t14530
}
