//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1195/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1195(t10514: f64, t18246: f64, t1006: f64, t750: f64, t2133: f64, t33: f64, t2433: f64, t821: f64, t2428: f64, t3202: f64, t9895: f64, t38: f64, t7679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18247 = t18246 * t10514;
    let t18250 = t1006 * t750;
    let t18254 = t33 * t2133;
    let t18265 = t33 * t2433;
    let t18268 = t1006 * t821;
    let t18271 = t33 * t2428;
    let t18295 = t9895 * t3202;
    let t18305 = t7679 * t38;
    (t18247, t18250, t18254, t18265, t18268, t18271, t18295, t18305)
}
