//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1093/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1093(t3228: f64, t5047: f64, t26896: f64, t1021: f64, t3448: f64, t1096: f64, t3452: f64, t1196: f64, t2825: f64, t1200: f64, t1189: f64, t3178: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26897 = t5047 * t3228;
    let t26898 = t26896 * t26897;
    let t26900 = t1021 * t3448;
    let t26902 = t1096 * t3452;
    let t26904 = t2825 * t1196;
    let t26906 = t2825 * t1200;
    let t26908 = t3178 * t1189;
    (t26897, t26898, t26900, t26902, t26904, t26906, t26908)
}
