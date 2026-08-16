//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1259/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1259(t648: f64, t670: f64, t1353: f64, t1448: f64, t3829: f64, t566: f64, t1408: f64, t240: f64, t828: f64, t9954: f64, t3935: f64, t1398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13435 = t648 * t670;
    let t13625 = t1353 * t1448;
    let t13656 = t3829 * t566;
    let t13767 = t1408 * t240;
    let t13783 = t9954 * t828;
    let t13789 = t3935 * t828;
    let t13791 = t1353 * t1398;
    (t13435, t13625, t13656, t13767, t13783, t13789, t13791)
}
