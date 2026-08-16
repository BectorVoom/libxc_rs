//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 715/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk715(t14140: f64, t2046: f64, t7297: f64, t1338: f64, t2039: f64, t638: f64, t669: f64, t2050: f64, t2128: f64, t31: f64, t13823: f64, t34796: f64, t7756: f64) -> (f64, f64, f64, f64) {
    let t70078 = t2046 * t7297 * t14140;
    let t70082 = t638 * t2039 * t669 * t1338;
    let t70086 = t2046 * t2050 * t2128 * t31;
    let t70100 = t13823 * t34796 * t7756;
    (t70078, t70082, t70086, t70100)
}
