//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 513/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk513(t14125: f64, t1972: f64, t14131: f64, t270: f64, t669: f64, t2039: f64, t638: f64, t31: f64, t2046: f64, t2050: f64, t3157: f64, t6477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14132 = t14125 * t1972;
    let t14133 = t14131 * t14132;
    let t14136 = t669 * t270;
    let t14138 = t638 * t2039 * t14136;
    let t14140 = t669 * t31;
    let t14142 = t2046 * t2050 * t14140;
    let t14144 = t6477 * t3157;
    (t14132, t14133, t14136, t14138, t14140, t14142, t14144)
}
