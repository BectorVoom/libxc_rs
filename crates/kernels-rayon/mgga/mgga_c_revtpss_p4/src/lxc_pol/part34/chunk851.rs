//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 851/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk851(t14242: f64, t4101: f64, t1432: f64, t2470: f64, t5763: f64, t3920: f64, t5603: f64, t2435: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64) -> (f64, f64, f64, f64, f64) {
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    (t14243, t14252, t14280, t14290, t14294)
}
