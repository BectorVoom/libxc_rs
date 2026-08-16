//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 446/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk446(t1821: f64, t224: f64, t1691: f64, t712: f64, t720: f64, t695: f64, t124: f64, t219: f64, t201: f64, t200: f64, t685: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1982 = t224 * t1821;
    let t1983 = t1982 * t1691;
    let t1986 = t712 * t720;
    let t1987 = t1986 * t695;
    let t1990 = t124 * t219;
    let t2000 = t124 * t201;
    let t2005 = 1.0_f64 / t685 / t200;
    let t2006 = t63 * t2005;
    (t1982, t1983, t1986, t1987, t1990, t2000, t2005, t2006)
}
