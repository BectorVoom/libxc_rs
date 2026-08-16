//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1991/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1991(t13026: f64, t65: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64, t4343: f64, t890: f64, t1544: f64, t2408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t57549 = t65 * t13026;
    let t60221 = t4171 * t2246;
    let t60224 = t1466 * t10308;
    let t60248 = t13267 * t602;
    let t61102 = t4343 * t890;
    let t61155 = t1544 * t2408;
    (t57549, t60221, t60224, t60248, t61102, t61155)
}
