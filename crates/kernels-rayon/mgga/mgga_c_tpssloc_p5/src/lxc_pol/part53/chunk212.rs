//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 212/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk212(t240: f64, t841: f64, t812: f64, t200: f64, t243: f64, t241: f64, t67: f64, t776: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t842 = t841 * t240;
    let t843 = t812 * t842;
    let t845 = 1.0_f64 / t243 / t200;
    let t847 = t241 * t845 * t67;
    let t849 = t847 * t820 * t776;
    (t842, t843, t845, t847, t849)
}
