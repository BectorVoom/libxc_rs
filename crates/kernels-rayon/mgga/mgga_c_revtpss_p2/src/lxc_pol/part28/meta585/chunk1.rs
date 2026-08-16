//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2051/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2051(t1364: f64, t26075: f64, t786: f64, t2482: f64, t7262: f64, t814: f64, t9821: f64, t820: f64, t844: f64, t3940: f64, t596: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94411 = t786 * t26075 * t1364;
    let t94423 = t2482 * t7262 * t814;
    let t94424 = t94423 * t9821;
    let t94429 = t820 * t7262 * t844;
    let t94430 = t94429 * t3940;
    let t94443 = t2482 * t7269 * t596;
    (t94411, t94423, t94424, t94429, t94430, t94443)
}
