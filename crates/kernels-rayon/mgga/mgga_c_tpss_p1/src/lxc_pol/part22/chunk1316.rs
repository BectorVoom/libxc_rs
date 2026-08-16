//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1316/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1316(t10818: f64, t1705: f64, t935: f64, t18005: f64, t6134: f64, t2162: f64, t64007: f64, t3665: f64, t818: f64, t1379: f64, t2425: f64, t19733: f64, t5570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64050 = t1705 * t10818 * t935;
    let t64060 = t6134 * t18005;
    let t64063 = t64007 * t2162;
    let t64118 = t3665 * t818;
    let t64122 = t1379 * t2425;
    let t64135 = t19733 * t5570;
    (t64050, t64060, t64063, t64118, t64122, t64135)
}
