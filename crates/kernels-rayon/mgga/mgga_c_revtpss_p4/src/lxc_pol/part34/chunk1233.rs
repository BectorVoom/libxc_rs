//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1233/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1233(t25305: f64, t99380: f64, t2453: f64, t2458: f64, t7760: f64, t25331: f64, t27213: f64, t93190: f64, t99211: f64, t25374: f64, t98848: f64, t99403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99425 = t25305 * t99380;
    let t99435 = t2453 * t7760 * t2458;
    let t99456 = t27213 * t25331;
    let t99460 = t93190 * t99211;
    let t99463 = t98848 * t25374;
    let t99466 = t99403 * t25374;
    (t99425, t99435, t99456, t99460, t99463, t99466)
}
