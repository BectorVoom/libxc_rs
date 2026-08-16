//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 812/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk812(t39785: f64, t1338: f64, t2039: f64, t575: f64, t638: f64, t2046: f64, t7297: f64, t8490: f64, t1686: f64, t270: f64, t1692: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39786 = 0.30487649791575028314e-3_f64 * t39785;
    let t39789 = t638 * t2039 * t575 * t1338;
    let t39792 = t2046 * t7297 * t8490;
    let t39796 = t638 * t2039 * t1686 * t270;
    let t39797 = 0.30487649791575028314e-3_f64 * t39796;
    let t39800 = t638 * t2039 * t1692 * t270;
    let t39801 = 0.30487649791575028314e-3_f64 * t39800;
    let t39804 = t638 * t2039 * t535 * t1338;
    (t39786, t39789, t39792, t39797, t39801, t39804)
}
