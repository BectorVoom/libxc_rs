//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 499/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk499(t14039: f64, t3128: f64, t3119: f64, t13862: f64, t1996: f64, t202: f64, t217: f64, t1173: f64, t14011: f64, t14034: f64, t3113: f64, t4443: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14040 = t3128 * t14039;
    let t14041 = t14040 * t3119;
    let t14042 = t13862 * t1996;
    let t14043 = t14041 * t14042;
    let t14045 = t217 * t202;
    let t14046 = t14045 * t1173;
    let t14047 = t14046 * t3119;
    let t14048 = t14011 * t14034;
    let t14049 = t14047 * t14048;
    let t14051 = t3113 * t4443;
    (t14040, t14041, t14042, t14043, t14045, t14046, t14047, t14048, t14049, t14051)
}
