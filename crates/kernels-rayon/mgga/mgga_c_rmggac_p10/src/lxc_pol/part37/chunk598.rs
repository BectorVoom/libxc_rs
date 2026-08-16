//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 598/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk598(t15296: f64, t3144: f64, t3076: f64, t551: f64, t2044: f64, t12200: f64, t558: f64, t7273: f64, t262: f64, t570: f64, t3068: f64, t10570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15297 = t15296 * t3144;
    let t15301 = t3076 * t551;
    let t15302 = t2044 * t15301;
    let t15303 = t12200 * t15302;
    let t15305 = t3076 * t558;
    let t15306 = t2044 * t15305;
    let t15307 = t7273 * t15306;
    let t15309 = t262 * t570;
    let t15310 = t3068 * t15309;
    let t15311 = t10570 * t15310;
    (t15297, t15302, t15303, t15306, t15307, t15309, t15310, t15311)
}
