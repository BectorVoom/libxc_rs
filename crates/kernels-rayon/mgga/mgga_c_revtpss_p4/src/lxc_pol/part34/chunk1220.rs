//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1220/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1220(t25898: f64, t7925: f64, t94849: f64, t25953: f64, t27884: f64, t10073: f64, t25938: f64, t27836: f64, t7289: f64, t97925: f64, t2470: f64, t27872: f64) -> (f64, f64, f64, f64, f64) {
    let t97956 = t94849 * t25898 * t7925;
    let t97985 = t27884 * t25953;
    let t98003 = t10073 * t27836 * t25938;
    let t98011 = t7289 * t97925;
    let t98028 = t27872 * t2470;
    (t97956, t97985, t98003, t98011, t98028)
}
