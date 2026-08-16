//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 878/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk878(t14022: f64, t14027: f64, t1451: f64, t201: f64, t3112: f64, t13862: f64, t14032: f64, t75027: f64, t15075: f64, t30080: f64, t15382: f64, t498: f64, t515: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64) {
    let t75700 = t3112 * t1451 * t201 * t14022 * t14027;
    let t75703 = t14032 * t13862 * t75027;
    let t75705 = t30080 * t15075;
    let t75718 = 0.1064114997332445985e-4_f64 * t7230 * t7231 * t515 * t15382 * t498;
    (t75700, t75703, t75705, t75718)
}
