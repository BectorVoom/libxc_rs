//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 244/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk244(t2012: f64, t291: f64, t20: f64, t253: f64, t22: f64, t259: f64, t26: f64) -> (f64, f64, f64, f64, f64) {
    let t2013 = t2012 * t291;
    let t2016 = t253 * t20;
    let t2017 = t259 * t22;
    let t2018 = t2017 * t26;
    let t2019 = t2016 * t2018;
    (t2013, t2016, t2017, t2018, t2019)
}
