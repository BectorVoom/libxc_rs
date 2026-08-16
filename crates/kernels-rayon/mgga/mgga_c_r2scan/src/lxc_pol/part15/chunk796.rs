//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 796/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk796(t7007: f64, t88: f64, t41: f64, t1234: f64, t2266: f64, t2854: f64, t4791: f64, t4794: f64, t4798: f64, t4806: f64, t4992: f64, t6794: f64, t6966: f64, t6970: f64, t6973: f64, t6975: f64) -> (f64, f64) {
    let t7008 = t7007 * t88;
    let t7009 = t41 * t7008;
    let t7011 = t2266 * t2854 * t1234;
    let t7012 = 3.0_f64 * t7011;
    let t7013 = t6966 - t6970 - t4791 + t4794 + t4798 - t4806 - t6973 - 0.2363e1_f64 * t6794 + t6975 + t4992 - t7009 - t7012;
    (t7009, t7013)
}
