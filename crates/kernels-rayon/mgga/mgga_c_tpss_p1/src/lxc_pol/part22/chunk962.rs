//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 962/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk962(t10117: f64, t3277: f64, t3346: f64, t72: f64, t240: f64, t3245: f64, t520: f64, t3240: f64, t3251: f64, t3243: f64, t756: f64, t3247: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10118 = t10117 * t3277;
    let t10120 = t3346 * t72;
    let t10121 = t10120 * t240;
    let t10122 = t520 * t3245;
    let t10131 = t3240 * t3251;
    let t10137 = t756 * t3243;
    let t10138 = t10137 * t3247;
    (t10118, t10120, t10121, t10122, t10131, t10137, t10138)
}
