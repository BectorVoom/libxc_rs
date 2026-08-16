//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 577/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk577(t14319: f64, t14324: f64, t14505: f64, t14508: f64, t14511: f64, t14514: f64, t14518: f64, t14519: f64, t14520: f64, t15000: f64, t15002: f64, t15012: f64) -> f64 {
    let t15014 = t15000 - t14505 + t14508 - t14319 + t14324 - t14511 - t14514 + t14518 + 0.19957069503106347607e-1_f64 * t15002 + t14519 - t14520 + t15012;
    t15014
}
